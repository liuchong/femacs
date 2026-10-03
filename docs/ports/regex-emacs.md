# emacs/src/regex-emacs.c (5367 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 41
- [x] STORE_NUMBER  (line 344, static)
- [x] extract_number  (line 367, static)
- [x] extract_address  (line 374, static)
- [x] extract_number_and_incr  (line 386, static)
- [x] debug_putchar  (line 452, static)
- [x] print_fastmap  (line 466, static)
- [x] print_partial_compiled_pattern  (line 497, static)
- [x] print_compiled_pattern  (line 742)
- [x] print_double_string  (line 780, static)
- [x] re_wctype_parse  (line 1526)
- [x] re_iswctype  (line 1605)
- [x] re_wctype_to_bit  (line 1635, static)
- [x] extend_range_table_work_area  (line 1663, static)
- [x] regex_compile  (line 1710, static)
- [x] store_op1  (line 2710, static)
- [x] store_op2  (line 2720, static)
- [x] insert_op1  (line 2732, static)
- [x] insert_op2  (line 2747, static)
- [x] at_begline_loc_p  (line 2765, static)
- [x] at_endline_loc_p  (line 2801, static)
- [x] group_in_compile_stack  (line 2812, static)
- [x] forall_firstchar_1  (line 2833, static)
- [x] forall_firstchar  (line 3055, static)
- [x] analyze_first_fastmap  (line 3072, static)
- [x] analyze_first_null  (line 3244, static)
- [x] analyze_first  (line 3302, static)
- [x] re_compile_fastmap  (line 3332, static)
- [x] re_set_registers  (line 3361)
- [x] re_search  (line 3385)
- [x] re_search_2  (line 3418)
- [x] skip_one_char  (line 3721, static)
- [x] execute_charset  (line 3766, static)
- [x] mutually_exclusive_exactn  (line 3829, static)
- [x] mutually_exclusive_charset  (line 3867, static)
- [x] mutually_exclusive_one  (line 3933, static)
- [x] mutually_exclusive_p  (line 4035, static)
- [x] re_match_2  (line 4059)
- [x] unwind_re_match  (line 4075, static)
- [x] re_match_2_internal  (line 4084, static)
- [x] bcmp_translate  (line 5313, static)
- [x] re_compile_pattern  (line 5352)

## Macros defined — 120
- [x] AT_STRINGS_BEG  (line 3685, body 5079a5637913)
- [x] AT_STRINGS_END  (line 3686, body e08b1e3b155d)
- [x] BIT_ALNUM  (line 1376, body 70070b762760)
- [x] BIT_ALPHA  (line 1375, body bf3d90382fd9)
- [x] BIT_BLANK  (line 1379, body 85201ee4c106)
- [x] BIT_GRAPH  (line 1377, body bd60cfec3dba)
- [x] BIT_LOWER  (line 1370, body 6b865327cda5)
- [x] BIT_MULTIBYTE  (line 1374, body a6d133efc0d7)
- [x] BIT_PRINT  (line 1378, body f3504e0d733d)
- [x] BIT_PUNCT  (line 1371, body 60a17847c279)
- [x] BIT_SPACE  (line 1372, body 34dfcaaf6c38)
- [x] BIT_UPPER  (line 1373, body 9042d4228c01)
- [x] BIT_WORD  (line 1369, body 1cb7a2e9afbe)
- [x] BUFFER_SYNTAX  (line 51, body 32fbf02beaf5)
- [x] BUF_PUSH  (line 1210, body 0c546d4c6c20)
- [x] BUF_PUSH_2  (line 1218, body 31e70c3b39aa)
- [x] BYTEWIDTH  (line 184, body fe5dbbcea5ce)
- [x] CHARSET_BITMAP_SIZE  (line 420, body 0a3bd0fae9c7)
- [x] CHARSET_RANGE_TABLE  (line 429, body 14e7771f13a4)
- [x] CHARSET_RANGE_TABLE_BITS  (line 432, body 89b78dfa9f30)
- [x] CHARSET_RANGE_TABLE_END  (line 440, body 166fa13d3df7)
- [x] CHARSET_RANGE_TABLE_EXISTS_P  (line 423, body 0929db1dec0e)
- [x] CHECK_INFINITE_LOOP  (line 1050, body 7fa7aab18753)
- [x] CLEAR_RANGE_TABLE_WORK_USED  (line 1360, body ed8402bdf26f)
- [x] COMPILE_STACK_EMPTY  (line 1314, body 48b80a3609f3)
- [x] COMPILE_STACK_FULL  (line 1315, body 0939b44447a3)
- [x] COMPILE_STACK_TOP  (line 1318, body eedd0d0000ba)
- [x] DEBUG_COMPILES_ARGUMENTS  (line 773, body da39a3ee5e6b)
- [x] DEBUG_PRINT  (line 803, body da39a3ee5e6b)
- [x] DEBUG_PRINT_COMPILED_PATTERN  (line 804, body da39a3ee5e6b)
- [x] DEBUG_PRINT_DOUBLE_STRING  (line 805, body da39a3ee5e6b)
- [x] DEBUG_STATEMENT  (line 802, body da39a3ee5e6b)
- [x] ENSURE_FAIL_STACK  (line 983, body 7b1c1ba308ac)
- [x] EXTEND_BUFFER  (line 1253, body fbaac518f4e2)
- [x] EXTEND_RANGE_TABLE  (line 1332, body 77770c421ae9)
- [x] EXTRACT_CHARACTER  (line 408, body e6f998a1dc46)
- [x] EXTRACT_NUMBER  (line 363, body 75ca9108f6b5)
- [x] EXTRACT_NUMBER_AND_INCR  (line 382, body fb4f1df64123)
- [x] FAILURE_PAT  (line 977, body 23311a0c98b9)
- [x] FAILURE_STR  (line 978, body 793fd6c4ac79)
- [x] FAIL_STACK_EMPTY  (line 911, body 248bde660d2a)
- [x] FAIL_STACK_GROWTH_FACTOR  (line 939, body 1b6453892473)
- [x] FIRST_STRING_P  (line 181, body e42e104bdd0b)
- [x] FIXUP_ALT_JUMP  (line 1675, body 4eb35ffa1799)
- [x] FREE_RANGE_TABLE_WORK_AREA  (line 1354, body 7c8f6ccd107d)
- [x] FREE_STACK_RETURN  (line 1683, body b502a0d37e73)
- [x] GET_BUFFER_SPACE  (line 1205, body 76d067bd452d)
- [x] GET_CHAR_AFTER  (line 86, body bc2d4e5477b2)
- [x] GET_CHAR_BEFORE_2  (line 67, body cf7fbb21caf0)
- [x] GET_INTERVAL_COUNT  (line 1490, body dfcd88463efa)
- [x] GROW_FAIL_STACK  (line 941, body b2045b155644)
- [x] INIT_BUF_SIZE  (line 1202, body cb4e5208b4cd)
- [x] INIT_COMPILE_STACK_SIZE  (line 1312, body cb4e5208b4cd)
- [x] INIT_FAILURE_ALLOC  (line 882, body 91032ad7bbcb)
- [x] INIT_FAIL_STACK  (line 916, body 7931bb6130c4)
- [x] INSERT_JUMP  (line 1236, body ed914ac273fa)
- [x] INSERT_JUMP2  (line 1240, body 684f45761207)
- [x] ISALNUM  (line 125, body c2bb34339251)
- [x] ISALPHA  (line 131, body d8871c40f5cf)
- [x] ISBLANK  (line 113, body 0afbbecf8ba1)
- [x] ISCNTRL  (line 108, body 1c5803c1ab7e)
- [x] ISDIGIT  (line 107, body cb5b94f067f3)
- [x] ISGRAPH  (line 117, body 30b21fd17083)
- [x] ISLOWER  (line 136, body 89236eba9abb)
- [x] ISPRINT  (line 121, body e21c01c211c8)
- [x] ISPUNCT  (line 144, body ccb5a34aaf6d)
- [x] ISSPACE  (line 151, body fe5267057c1b)
- [x] ISUNIBYTE  (line 102, body 17705e8175cb)
- [x] ISUPPER  (line 138, body 47351c96b187)
- [x] ISWORD  (line 153, body fce85be20968)
- [x] ISXDIGIT  (line 109, body 6d11a3fede7c)
- [x] IS_REAL_ASCII  (line 99, body 9423745e6d3c)
- [x] MAX_BUF_SIZE  (line 1247, body d71c75f9f01d)
- [x] MAX_REGNUM  (line 1283, body 3028f51407d8)
- [x] NEXT_FAILURE_HANDLE  (line 979, body 2742ea662579)
- [x] NUM_NONREG_ITEMS  (line 974, body 77de68daecd8)
- [x] PATFETCH  (line 1187, body 9f114cd4921c)
- [x] POINTER_TO_OFFSET  (line 3648, body 3c3e67f99e4a)
- [x] POP_FAILURE_INT  (line 971, body 770a682abf30)
- [x] POP_FAILURE_POINT  (line 1128, body b800c2f99b75)
- [x] POP_FAILURE_POINTER  (line 970, body d5962bd20f7c)
- [x] POP_FAILURE_REG_OR_COUNT  (line 1026, body 30612f38054b)
- [x] POS_ADDR_VSTRING  (line 3393, body 3a263793ff3b)
- [x] PREFETCH  (line 3658, body 7593b430651a)
- [x] PREFETCH_NOLIMIT  (line 3676, body 0b2db723594b)
- [x] PUSH_FAILURE_INT  (line 965, body 59427f91e80f)
- [x] PUSH_FAILURE_POINT  (line 1080, body 68d541e2eec6)
- [x] PUSH_FAILURE_POINTER  (line 959, body 0fca7937dd92)
- [x] PUSH_FAILURE_REG  (line 996, body a7bfae640b83)
- [x] PUSH_NUMBER  (line 1012, body b168346bcf79)
- [x] RANGE_TABLE_WORK_BITS  (line 1363, body 1271d12e875f)
- [x] RANGE_TABLE_WORK_ELT  (line 1364, body d444a9996b05)
- [x] RANGE_TABLE_WORK_USED  (line 1362, body 2534cf9200b2)
- [x] REGEX_REALLOCATE  (line 174, body 3af67e88e992)
- [x] REGEX_USE_SAFE_ALLOCA  (line 170, body 7d64c88fa538)
- [x] REG_UNSET  (line 1165, body fc207782b800)
- [x] REMAINING_AVAIL_SLOTS  (line 1115, body 66da1b585521)
- [x] RETURN_CONSTRAIN  (line 3955, body de20b9279805)
- [x] RE_CHAR_TO_MULTIBYTE  (line 60, body 62934ff68cc3)
- [x] RE_CHAR_TO_UNIBYTE  (line 62, body 204cbb3a3020)
- [x] RE_DUP_MAX  (line 45, body 753f39de37a0)
- [x] RE_MULTIBYTE_P  (line 53, body 348b62bfeecc)
- [x] RE_STRING_CHAR  (line 55, body 6114f6c48917)
- [x] RE_STRING_CHAR_AND_LENGTH  (line 57, body 82ef2f8a5412)
- [x] RE_TARGET_MULTIBYTE_P  (line 54, body 97689ecfa74c)
- [x] RE_TRANSLATE  (line 1196, body 9a820b736756)
- [x] SETUP_ASCII_RANGE  (line 1397, body 6286e957c812)
- [x] SETUP_MULTIBYTE_RANGE  (line 1457, body 1ce5e0ae9f80)
- [x] SETUP_UNIBYTE_RANGE  (line 1417, body a5c6a2274a1f)
- [x] SET_LIST_BIT  (line 1383, body f4eb6061eb0f)
- [x] SET_RANGE_TABLE_WORK_AREA  (line 1346, body ded8d964b919)
- [x] SET_RANGE_TABLE_WORK_AREA_BIT  (line 1342, body a289574b1582)
- [x] STORE_CHARACTER_AND_INCR  (line 397, body a2898d734c4b)
- [x] STORE_JUMP  (line 1228, body 4a83d7cadabd)
- [x] STORE_JUMP2  (line 1232, body f2006d7f10c3)
- [x] STORE_NUMBER_AND_INCR  (line 354, body ea0190535d40)
- [x] SYNTAX  (line 48, body 61d123e5ded5)
- [x] TOP_FAILURE_HANDLE  (line 980, body bb45f9a1ac8f)
- [x] TRANSLATE  (line 1197, body ed185c72a346)
- [x] TYPICAL_FAILURE_SIZE  (line 1112, body 91032ad7bbcb)

## Types and their fields — 4
- [x] anafirst_data: bool multibyte, char* fastmap, bool match_any_multibyte_characters
- [x] fail_stack_elt: re_char* pointer, intptr_t integer
- [x] mutexcl_data: struct re_pattern_buffer* bufp, re_char* p1, bool unconstrained
- [x] range_table_work_area: int* table, int allocated, int used, int bits

## File-scope objects — 3
- [x] ptrdiff_t emacs_re_safe_alloca  (line 168)
- [x] static int regex_emacs_debug  (line 768, static)
- [x] ptrdiff_t emacs_re_max_failures  (line 893)
## Decisions

**Complete (2026-10-04).**  Every one of the ledger's 168 rows is ticked, the unit compiles
(`built src/regex-emacs.rs`), and `tools/extract-abi.py --check emacs/src/regex-emacs.c
femacs/src/regex-emacs.rs` prints `OK`.

**Dead arms: carried, not recorded (status 3.49/P-51).**  `src/regex-emacs.c` is *not* in
`docs/baseline/config/cond-eval-verified.txt`: `tools/check-cond-eval.py src/regex-emacs.c`
prints `15 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]` (C 42's `#ifdef RE_DUP_MAX`
- clang False, the table True, because `RE_DUP_MAX` is a system `regex.h` macro the table
knows and the `#undef` that follows is what removes it).  Both arms of every `#if` are
therefore carried:
* C 42-44, `#ifdef RE_DUP_MAX` / `# undef RE_DUP_MAX`: whether the `# undef` runs or not, C 45
  defines the value, so one `pub const RE_DUP_MAX: c_int = 0xffff;` answers for both arms.
* C 446-800, `#if defined REGEX_EMACS_DEBUG || ENABLE_CHECKING` (the printers): the whole
  block is `#[cfg (any (regex_emacs_debug, enable_checking))]`.  `ENABLE_CHECKING` is undef
  in the frozen `config.h` (macros.tsv 872), so clang compiles none of it;
  `--cfg enable_checking` compiles all of it, which the per-file loop requires (verified:
  the materialized unit compiles under `--cfg enable_checking`, exit 0).
* C 766-807, `#ifdef REGEX_EMACS_DEBUG` (the counter and the four `DEBUG_*` macros):
  `#[cfg (regex_emacs_debug)]` for the live text and `#[cfg (not (regex_emacs_debug))]` for
  the `#else` arm's empty definitions.  `REGEX_EMACS_DEBUG` is a source-level macro, not a
  `config.h` key, so it is not a `xtask dead-arms` key either.
* C 136-140 and 2858-2860, 2953-2955, 2968-2970, 3022-3026, 3236-3238, 3283-3285, 4024-4026
  (`fprintf`/`abort` under `ENABLE_CHECKING`), each under `#[cfg (enable_checking)]`.
* C 3698-3713, the literal `#if 0` (`WORDCHAR_P`/`AT_WORD_BOUNDARY`): dead in *every*
  configuration, so it is recorded here only (the `#if 0` rule of P-08, the
  search.rs/intervals.c precedent) and the port writes nothing.
`print_compiled_pattern` is non-`static` in C but lives inside the checking block, so the
definition is gated the same way: in the frozen build the unit emits no such symbol (§3.26).

**The `regex-emacs.h` mirror lands with this unit** (the `docs/ports/thread.md` precedent):
`struct re_pattern_buffer`, its six bit accessors, the `RE_PATTERN_*` masks and the eight
declarations are in `femacs/src/include/regex-emacs.rs`, the declarations in
`pub mod regex_emacs_decls` + glob import because this unit defines them (PORTING.md 0.1-10:
a same-named declaration plus definition in one unit is an `E0428`).  `src/search.c`'s port
carried its own copy of the record, the masks, the accessors, the five `re_*` prototypes and
`print_compiled_pattern`; those copies are deleted in the same commit, and `src/search.c`
rebuilds and `--check`s `OK` with the mirror's text (`femacs/src/search.rs` 3,846 -> 3,722
lines).  `src/syntax.c`, `src/thread.c` and `src/emacs.c` - the mirror's other consumers -
also rebuild after the change.

**Use-site declarations (mirror gaps).**  Three: `<stdio.h>`'s `fprintf` and `putc` (the
mirror carries `fputs`/`fputc`; C 449's `sysstdio.h` chain is where C gets all of them),
`extern FILE *stderr` (Darwin's `__stderrp`), and `dispnew.h` 603's
`update_redisplay_ticks (int, struct window *)`, whose window is reached only as a pointer and
so is the ABI-equal `*mut c_void` (PORTING.md 0.1-2/P-72, the bytecode.rs shape).  Nothing
else is restated: `regex-emacs.h`, `character.h`, `buffer.h`, `syntax.h`, `category.h` and
`dispextern.h` are all mirrored that far.

**Statement macros that capture a local (PORTING.md 2.2/P-57).**  A `macro_rules!` resolves
locals at its *definition* site, so the 14 macros that name `regex_compile`'s locals
(`TRANSLATE`, `PATFETCH`, `GET_BUFFER_SPACE`, `BUF_PUSH`, `BUF_PUSH_2`, `INSERT_JUMP`,
`INSERT_JUMP2`, `EXTEND_BUFFER`, `FIXUP_ALT_JUMP`, `FREE_STACK_RETURN`,
`COMPILE_STACK_EMPTY/FULL/TOP`, `SET_LIST_BIT`, the three `SETUP_*_RANGE` macros and
`GET_INTERVAL_COUNT`) are defined *inside* `regex_compile`, and the 19 that name
`re_match_2_internal`'s (`REGEX_REALLOCATE`, `INIT_FAIL_STACK`, `GROW_FAIL_STACK`,
`PUSH_FAILURE_POINTER/INT/REG/POINT`, `POP_FAILURE_POINTER/INT/REG_OR_COUNT`,
`FAILURE_PAT/STR`, `NEXT_FAILURE_HANDLE`, `TOP_FAILURE_HANDLE`, `ENSURE_FAIL_STACK`,
`REMAINING_AVAIL_SLOTS`, `CHECK_INFINITE_LOOP`, `POP_FAILURE_POINT`, `GET_CHAR_BEFORE_2`,
`GET_CHAR_AFTER`, `POINTER_TO_OFFSET`, `AT_STRINGS_BEG/END`, `FIRST_STRING_P`, `PREFETCH`,
`PREFETCH_NOLIMIT`, `POS_ADDR_VSTRING`) inside *that* function, after the locals they
capture; their C comments stay at C's own positions with a note.  Three consequences, each
recorded in `docs/ports/macro-body-notes.tsv`:
* `REGEX_REALLOCATE`'s C body assigns the enclosing block's `destination`, which Rust cannot
  resolve at the definition site, so the port hands it in as the first metavariable (and
  `GROW_FAIL_STACK`/`ENSURE_FAIL_STACK` pass it along).
* C's `REGEX_USE_SAFE_ALLOCA`/`INIT_FAIL_STACK ()` and `count` declare three names the
  failure-stack macros capture; their *declarations* are hoisted above the macro block
  (`sa_avail`, `sa_count`, `count`) and C's own statements keep their positions
  (`sa_avail = emacs_re_safe_alloca`, `INIT_FAIL_STACK ()`).
* `bcmp_translate` is not inside `re_match_2_internal`, so it carries its own `GET_CHAR_AFTER`
  copy - there `target_multibyte` is a parameter, not a local.

**`goto` and labels.**  `regex_compile`'s `goto normal_char` (six sites) and
`goto unfetch_interval` are the `goto_normal_char`/`goto_unfetch_interval` flags, evaluated
where C's labels are; `forall_firstchar_1`'s `goto do_twoway_jump`/`goto do_jump` are its
`stage` flag; `re_search_2`'s two `goto advance` are a `'advance` labelled block;
`re_match_2_internal`'s `goto fail` (28 sites), `goto restore_best_regs` (two sites),
`goto succeed_label`, `goto on_failure`, `goto unconditional_jump` and
`goto continue_failure_jump` are the `goto_fail`/`do_restore`/`do_succeed`/`goto_on_failure`/
`goto_unconditional_jump`/`goto_continue_failure_jump` flags plus one `fallthrough` flag for
the nastyloop arm, all evaluated where C's labels are (PORTING.md 2.2, the
ccl.rs/bytecode.rs shape).  `PREFETCH` sets `goto_fail` where C's `goto fail` is, and every
call site tests it before continuing - that is what C's `goto` does.

**Spellings that stay closest to C.**  `re_char` is a type alias for `u8` (C's
`const unsigned char`), `re_opcode_t`/`reg_errcode_t`/`regnum_t`/`pattern_offset_t`/
`re_wctype_t` are integer aliases with one `pub const` per C enum member (values in C's
order); the IS* predicates keep C's int-returning shape (`(cond) as c_int`) so that
`ISALNUM (ch) != 0` reads as C's line does; pointer indexing is `p.offset`/`offset_from` and
C's `p++`/`nchars++` are `p = p.offset (1)`/`nchars += 1` (Rust has no postfix increment);
`memcpy`/`memcmp`/`memset`/`strlen` keep their C names; the bit members of
`re_pattern_buffer` and `buffer_text` are read and written through the mirror's accessors and
masks, which is why the `can_be_null`/`regs_allocated`/`fastmap_accurate`/`used_syntax`/
`multibyte`/`target_multibyte` and `inhibit_shrinking` families in
`docs/ports/token-aliases.tsv` are occurrence-checked here too.

**Measured layout** (`clang -std=gnu23`, the frozen target): `struct fail_stack_elt` 8
(union), `fail_stack_type` 32, `compile_stack_elt_t` 32, `compile_stack_type` 24,
`range_table_work_area` 24, `anafirst_data` 24 (`multibyte`@0, `fastmap`@8,
`match_any_multibyte_characters`@16) and `mutexcl_data` 24 (`bufp`@0, `p1`@8,
`unconstrained`@16); the `const _` assertions above pin those numbers.  `struct
re_pattern_buffer`'s ten numbers are restated from the mirror's own assertions
(docs/ports/search.md measured them; three units now agree).

**Line-count relation:** C 5,367 lines (all live for this ledger) against **6,189 Rust
lines** (15.3%), declared as the `total`-basis row `regex-emacs 0.16` in
`docs/ports/line-drift-tolerance.tsv` with the reason (both `#if` arms carried; 120 macro
bodies written as multi-line Rust blocks; 19 macros defined inside their function; the
use-site declarations, layout assertions, `match` arms, labelled blocks and the
`p.offset (1)`/`nchars += 1` statements).  The per-function token comparison - the real
guard - is `OK`.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call the reference does not make: the mirror
expansions (`STORE_NUMBER_AND_INCR`, `EXTRACT_*`, `BUF_PUSH*`, `INSERT_JUMP*`, `SETUP_*_RANGE`,
`PUSH_FAILURE_*`, `POP_FAILURE_*`, `SAFE_*`, `DEBUG_*`, `IS*`/`GET_CHAR_*`) are C's own macros
expanding at the same points, the bit-member accessors are the packed fields C reads, and
`REGEX_REALLOCATE`/`GROW_FAIL_STACK`/`ENSURE_FAIL_STACK` pass the one `destination` C's
enclosing block declares instead of assigning that name.  The mechanical check: the set of
callee names in the port minus the set in the reference is exactly Rust's own vocabulary
(`repr`/`derive`/`fn`/`unsafe`/`as_ptr`/`into`/`offset`/`offset_from`/`is_null`/`null`/
`null_mut`/`write`/`mut`/`any`/`not`/`some`), the mirror's bit accessors, `emacs_abort`
(C's `abort`), `qnil` (C's `Qnil`), `wrapping_add` (C's `+=`) and
`overflowing_add`/`overflowing_mul` (C's `ckd_add`/`ckd_mul`, the token-aliases families) -
no invented callee.
