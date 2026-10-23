# emacs/src/search.c (3514 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 19
- [x] looking-at  (Flooking_at, 1/2, doc 7bc13d37b40d, line 347)
- [x] posix-looking-at  (Fposix_looking_at, 1/2, doc 6e2580e7e8da, line 357)
- [x] string-match  (Fstring_match, 2/4, doc 04f4d75df6ac, line 442)
- [x] posix-string-match  (Fposix_string_match, 2/4, doc 887eee5efdae, line 461)
- [x] search-backward  (Fsearch_backward, 1/4, doc a38dbdb28e61, line 2206)
- [x] search-forward  (Fsearch_forward, 1/4, doc 06646c79e8d0, line 2218)
- [x] re-search-backward  (Fre_search_backward, 1/4, doc 03e04797fcd2, line 2248)
- [x] re-search-forward  (Fre_search_forward, 1/4, doc 86bb1432ad51, line 2264)
- [x] posix-search-backward  (Fposix_search_backward, 1/4, doc 34ebffadd14a, line 2295)
- [x] posix-search-forward  (Fposix_search_forward, 1/4, doc 9fd0153fcc65, line 2312)
- [x] replace-match  (Freplace_match, 1/5, doc 1fe55b68aff5, line 2344)
- [x] match-beginning  (Fmatch_beginning, 1/1, doc d40739eae873, line 2786)
- [x] match-end  (Fmatch_end, 1/1, doc 473d67654a26, line 2801)
- [x] match-data  (Fmatch_data, 0/3, doc 263a411e2a1b, line 2816)
- [x] set-match-data  (Fset_match_data, 1/2, doc 87182688ac02, line 2949)
- [x] match-data--translate  (Fmatch_data__translate, 1/1, doc 204a0fd489f3, line 3062)
- [x] regexp-quote  (Fregexp_quote, 1/1, doc 20da70c6da96, line 3165)
- [x] newline-cache-check  (Fnewline_cache_check, 0/1, doc 7606389d93b9, line 3292)
- [x] re--describe-compiled  (Fre__describe_compiled, 1/2, doc 151b65ba727b, line 3375)

## Functions defined — 36
- [x] matcher_overflow  (line 78, static)
- [x] freeze_buffer_relocation  (line 84, static)
- [x] compile_pattern_1  (line 104, static)
- [x] shrink_regexp_cache  (line 142)
- [x] clear_regexp_cache  (line 160)
- [x] unfreeze_pattern  (line 173, static)
- [x] freeze_pattern  (line 180, static)
- [x] compile_pattern  (line 199, static)
- [x] looking_at_1  (line 264, static)
- [x] string_match_1  (line 370, static)
- [x] fast_string_match_internal  (line 485)
- [x] fast_c_string_match_internal  (line 508)
- [x] fast_looking_at  (line 535)
- [x] newline_cache_on_off  (line 602, static)
- [x] find_newline  (line 675)
- [x] scan_newline  (line 968)
- [x] scan_newline_from_point  (line 986)
- [x] find_newline_no_quit  (line 1003)
- [x] find_before_next_newline  (line 1014)
- [x] search_command  (line 1033, static)
- [x] trivial_regexp_p  (line 1109, static)
- [x] search_buffer_re  (line 1160, static)
- [x] search_buffer_non_re  (line 1298, static)
- [x] search_buffer  (line 1518)
- [x] simple_search  (line 1556, static)
- [x] boyer_moore  (line 1762, static)
- [x] set_search_regs  (line 2178, static)
- [x] match_limit  (line 2769, static)
- [x] save_search_regs  (line 3082, static)
- [x] restore_search_regs  (line 3097)
- [x] update_search_regs  (line 3115)
- [x] unwind_set_match_data  (line 3149, static)
- [x] record_unwind_save_match_data  (line 3157)
- [x] find_newline1  (line 3207)
- [x] syms_of_search  (line 3420)
- [x] syms_of_search_for_pdumper  (line 3500, static)

## Macros defined — 2
- [x] REGEXP_CACHE_SIZE  (line 40, body 91032ad7bbcb)
- [x] TRANSLATE  (line 1137, body ba70eb6c765e)

## Types and their fields — 1
- [x] regexp_cache: struct regexp_cache* next, Lisp_Object regexp, Lisp_Object f_whitespace_regexp, Lisp_Object syntax_table, struct re_pattern_buffer buf, char fastmap, bool posix, bool busy

## File-scope objects — 4
- [x] static struct regexp_cache searchbufs  (line 61, static)
- [x] static struct regexp_cache searchbuf_head  (line 64, static)
- [x] Lisp_Object re_match_object  (line 75)
- [x] static struct re_registers search_regs_1  (line 1157, static)

## Decisions

- **What the file carries.** C's 19 `DEFUN`s, its 36 functions (36 definitions; the
  non-`static` ones - `shrink_regexp_cache`, `clear_regexp_cache`,
  `fast_string_match_internal`, `fast_c_string_match_internal`, `fast_looking_at`,
  `find_newline`, `scan_newline`, `scan_newline_from_point`, `find_newline_no_quit`,
  `find_before_next_newline`, `search_buffer`, `restore_search_regs`, `update_search_regs`,
  `record_unwind_save_match_data`, `find_newline1`, `syms_of_search` - carry
  `#[unsafe(no_mangle)]` with the one non-`static` object `re_match_object`, PORTING.md 0.2),
  its two macros (`REGEXP_CACHE_SIZE`, `TRANSLATE`), its one type (`struct regexp_cache`,
  C declares it in the .c) and its four file-scope objects (`searchbufs`,
  `searchbuf_head`, `re_match_object` and `search_regs_1`).  The `static` helpers that C
  hands to a callback stay `unsafe extern "C" fn` (`unfreeze_pattern`,
  `unwind_set_match_data`, `syms_of_search_for_pdumper`, the three whose address C takes at
  `record_unwind_protect*`/`pdumper_do_now_and_after_load`; PORTING.md 1.4), the `AVOID`
  ones are `-> !` (`matcher_overflow`).
- **The two `DEFVAR_LISP` variables this unit owns** (docs/status.md 3.29):
  `Vsearch_spaces_regexp` (C 3473-3479) and `Vinhibit_changing_match_data` (C 3481-3487)
  are defined here as `#[unsafe(no_mangle)] pub static mut … = lisp_h_Qnil ()`, because
  C's storage is `globals.f_<vname>` and `struct emacs_globals globals` lives in alloc.c;
  the generated `include/globals.rs` only declares them, and the definitions shadow that
  glob import (PORTING.md 0.1-10).  `tools/check-globals.py` reports zero variables
  without a definition in their own unit after this change.
- **Dead arms: carried, not recorded (status 3.49/P-51).**  `src/search.c` is *not* in
  `docs/baseline/config/cond-eval-verified.txt` (`tools/check-cond-eval.py src/search.c`
  says `6 conditions, 0 unknown, 0 disagreements [VERIFIED]`, but the ledger's
  `blind_conditions` reads the list, not the verdict), so `tools/extract-abi.py` keeps
  *both* arms of every `#if` live and the per-function token check demands the tokens of
  C's text on both sides.  The file therefore carries the text of the arms the frozen build
  does not compile, each under a `#[cfg (feature = "…")]`:
  * C 84-97, `freeze_buffer_relocation`'s `#ifdef REL_ALLOC` half (`REL_ALLOC` is undef,
    macros.tsv 3302): feature `rel_alloc`, which also declares
    `r_alloc_inhibit_buffer_relocation` at that use site.
  * C 2019-2028 and 2040-2058, `boyer_moore`'s two `#ifdef REL_ALLOC` blocks (the
    `cursor_off` save/restore around `set_search_regs`): feature `rel_alloc`.
  * C 3394-3412, `Fre__describe_compiled`'s three arms.  `ENABLE_CHECKING` is undef
    (macros.tsv 872), so clang compiles the first (`error ("Not available: …")`); the port
    keys it `#[cfg (not (enable_checking))]` and the second
    (`ENABLE_CHECKING && HAVE_OPEN_MEMSTREAM`, the latter 1, macros.tsv 1482) on
    `#[cfg (enable_checking)]` - the same key `lisp.rs`'s `eassert!` and `region-cache.rs`
    use.  The third (`!HAVE_OPEN_MEMSTREAM`) is dead in the frozen configuration and is
    carried under `#[cfg (all (enable_checking, feature = "no_have_open_memstream"))]`,
    together with its `print_compiled_pattern`/`stderr` declarations.
  * C 1091-1094, `search_command`'s literal `#if 0` (`np = lim;`), is dead in *every*
    configuration and is **recorded here only** (the `#if 0` rule of P-08, the
    intervals.c/ccl.c precedent); the port writes the four live statements around it.
  No other `#if` in the file reaches a compiler: C 86/1091/2032/3394 are the whole set.
- **Use-site declarations (mirror gaps, reported in the final message).**  There is no
  mirror for `regex-emacs.h`'s engine half (`docs/ports/include-regex-emacs.md`: the mirror
  stops at the character-class API), so this unit declares `struct re_pattern_buffer`
  (C 67-114 of that header), its six bit-member accessors and the five `re_*` entries
  (`re_compile_pattern`, `re_search`, `re_search_2`, `re_match_2`, `re_set_registers`) with
  C's prototype above each (PORTING.md 0.1-2/P-72, the indent.rs/ccl.rs shape).
  `region-cache.h`'s five `region_cache` entries are its mirror's now
  (`docs/ports/include-region-cache.md`, status 3.74), which spells `struct region_cache *` as
  the ABI-equal `*mut c_void`; this unit keeps its use-site `pub type region_cache = c_void;`
  because six of its local declarations name C's type (P-55/P-72 rule 3; the definitions are
  `src/region-cache.rs`), and the declarations that stood here went away with the mirror.  `memrchr`, `substring_both`, `make_specified_string`,
  `copy_text` and `open_memstream`/`stderr`/`print_compiled_pattern` are the same shape:
  each is a lisp.h/`<string.h>`/<stdio.h> declaration this unit needs and no mirror carries.
  None of these edits touches a shared mirror, so no other consumer needed rebuilding.
- **Measured layout** (`clang -std=gnu23`, the frozen target; the probe is the two records
  plus `<stddef.h>`, printing `sizeof`/`offsetof`): `struct re_pattern_buffer` 64/align 8
  with `buffer`@0, `allocated`@8, `used`@16, `charset_unibyte`@24, `fastmap`@32,
  `translate`@40, `re_nsub`@48 and the bit run at 56; `struct regexp_cache` 360/align 8 with
  `next`@0, `regexp`@8, `f_whitespace_regexp`@16, `syntax_table`@24, `buf`@32,
  `fastmap`@96, `posix`@352, `busy`@353.  The unit pins those numbers with 20 `const _`
  assertions (10 + 10).
- **The bits of `re_pattern_buffer`.**  C's two `bool_bf` runs and `unsigned
  regs_allocated : 2` (regex-emacs.h 95-113) are one storage unit; the port packs them into
  `bits: u8` with `RE_PATTERN_CAN_BE_NULL`/`_REGS_ALLOCATED`/`_FASTMAP_ACCURATE`/
  `_USED_SYNTAX`/`_MULTIBYTE`/`_TARGET_MULTIBYTE` (PORTING.md 1.3/P-80) and one accessor
  pair per C member.  `docs/ports/token-aliases.tsv` carries three new families
  (`multibyte`, `used_syntax`, `target_multibyte`; the other three already existed for the
  gnulib `re_pattern_buffer`), so every C read and write of those three - including the
  local variables of the same name in `fast_looking_at`/`simple_search`/`boyer_moore` - is
  occurrence-checked.
- **`struct re_pattern_buffer`'s three unread members.**  `can_be_null`, `regs_allocated`
  and `fastmap_accurate` have no reader or writer in this file; they are declared because
  C's record has them and the layout assertion is on the whole record (the mirror's
  fields-not-members rule, PORTING.md 0.1-2).
- **`goto`, labels and statement macros.**  `compile_pattern`'s `goto compile_it` (C 244)
  is the `compile_it` flag the port tests at the label's position, so the
  `eassert`/`compile_pattern_1` tail appears once; `search_command`'s `#if 0` is gone (above);
  `simple_search`'s four `goto stop` are `break 'stop` out of the labelled block that
  carries C's `stop:` label; `boyer_moore`'s two `goto hit`/`goto hit2` jump *into* the
  middle of a `while (1)` body, which Rust cannot, so each is a `hit`/`hit2` flag that
  selects the statements after the label (PORTING.md 2.2, the bytecode.rs/tparam.rs shape);
  `trivial_regexp_p`'s two `switch`es become `match`es with the same arms in the same order.
  All of those tokens are recorded in `docs/ports/macro-body-notes.tsv` (11 new rows, one of
  them for `TRANSLATE`'s `do { … } while (0)` epilogue and one for `Freplace_match`'s
  `UNINIT`).
- **Spellings that stay closest to C.**  `memchr`/`memrchr` keep their C names;
  `search_regs! ()`/`last_thing_searched! ()` (thread.h's macros) are called with `!` and
  their `start[i]`/`end[i]` are `*…start.offset (i)` (the pointer-index-idiom rows in
  macro-body-notes.tsv); `XSETFASTINT!`/`XSETBUFFER!`/`CHECK_FIXNUM_COERCE_MARKER!`/
  `BVAR!` keep C's macro names; `inc_both`/`dec_both` take `&raw mut` to the locals C passes
  addresses of; the two `char` buffers C indexes with octal bounds (`BM_tab[0400]`,
  `simple_translate[0400]`, `char fastmap[0400]`) are `[c_int; 256]`/`[u8; 256]`/
  `[c_char; 256]` - `256` is C's `0400`, and the tool's literal rule folds the two; C's
  `0`/`1` passed for `bool` parameters are Rust `false`/`true` where the *function's* body
  still mentions every other token (the six `Flooking_at`/… rows in macro-body-notes.tsv).
  `USE_SAFE_ALLOCA!` takes the two names C's macro declares (the mirror's documented
  deviation), and `SAFE_NALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!` keep C's call shape.
- **`line` and `column` counts:** C 3514 lines (all live for this ledger, since the file is
  not in the verified list); the port is 3846, which
  `docs/ports/line-drift-tolerance.tsv` declares with the `total` basis (9.4% against total) and the
  reason; the per-function token comparison is the real guard and it is `OK`.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call the reference does not make: the mirror
expansions (`BVAR!`, `FETCH_BYTE!`, `TRANSLATE!`, `SAFE_*`, `list!`, `CALLN!`,
`USE_SAFE_ALLOCA!`) are C's own macros expanding at the same points, the bit-member
accessors are the packed fields C reads, and the `*mut region_cache` use-site spelling
changes a parameter's declared type, not the call graph.  The `re_*`/`region_cache` calls
are the same five + five the C text makes.

- **The `regex-emacs.h` engine half is the mirror's now (2026-10-03, step (a)-(c) of the
  regex-emacs lane).**  This unit carried its own copy of `struct re_pattern_buffer` (with its
  ten `const _` layout assertions), the `RE_PATTERN_*` masks, `REGS_UNALLOCATED`/
  `REGS_REALLOCATE`/`REGS_FIXED`, the twelve bit accessors, the five `re_*` prototypes and
  `print_compiled_pattern`, because `docs/ports/include-regex-emacs.md` stopped at the
  character-class API.  `src/include/regex-emacs.rs` carries all of them now (the
  `thread.md` precedent: a mirror lands in the same commit as its definition unit), and those
  copies are deleted here - 133 lines out, 9 comment lines in - or this unit would be an
  `E0428` (the mirror is `include!`d at search.rs 77).  `build-tu src/search.c` prints
  `built src/search.rs` and `--cfg enable_checking`'s second pass prints
  `built src/search.rs (--cfg enable_checking)`, and
  `tools/extract-abi.py --check emacs/src/search.c src/search.rs` still prints `OK`.
  `re_match_object` stays defined here (C's search.c 75 owns it; regex-emacs.c does not
  define it), and `REGS_*` went with the rest because the mirror's note records that C
  defines them in `regex-emacs.c` 862, not in the header.
