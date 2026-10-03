# emacs/src/ccl.c (2394 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 5
- [x] ccl-program-p  (Fccl_program_p, 1/1, doc 9aa179c90221, line 2029)
- [x] ccl-execute  (Fccl_execute, 2/2, doc 6e65496f40a9, line 2050)
- [x] ccl-execute-on-string  (Fccl_execute_on_string, 3/5, doc b808a73f354c, line 2097)
- [x] register-ccl-program  (Fregister_ccl_program, 2/2, doc 963bc6cfbb19, line 2245)
- [x] register-code-conversion-map  (Fregister_code_conversion_map, 2/2, doc 26a5fa8e7d96, line 2313)

## Functions defined — 7
- [x] ccl_debug_hook  (line 850)
- [x] GET_TRANSLATION_TABLE  (line 869, static)
- [x] ccl_driver  (line 875)
- [x] resolve_symbol_ccl_program  (line 1885, static)
- [x] ccl_get_compiled_code  (line 1956, static)
- [x] setup_ccl_program  (line 1997)
- [x] syms_of_ccl  (line 2361)

## Macros defined — 91
- [x] ADDR  (line 931, body b866f35c4c64)
- [x] ASCENDING_ORDER  (line 743, body 0b9ceb37a6ad)
- [x] CCL_AND  (line 675, body 2fee0d789cea)
- [x] CCL_Branch  (line 219, body 83ff6ba2dee2)
- [x] CCL_CALL_FOR_MAP_INSTRUCTION  (line 620, body e62c0a99d39b)
- [x] CCL_CODE_MAX  (line 86, body 3bc315fbb9d7)
- [x] CCL_CODE_MIN  (line 87, body 71b594d231ac)
- [x] CCL_Call  (line 284, body c9e339801005)
- [x] CCL_DEBUG_BACKTRACE_LEN  (line 845, body dd7c1a3d9d56)
- [x] CCL_DECODE_CHAR  (line 814, body 20d15610dd0a)
- [x] CCL_DECODE_SJIS  (line 690, body a8ec1c1966ef)
- [x] CCL_DIV  (line 673, body 9fc6fa67f2a4)
- [x] CCL_DIVMOD  (line 682, body c78b5e6fa049)
- [x] CCL_ENCODE_CHAR  (line 822, body 2c1a9570f97c)
- [x] CCL_ENCODE_SJIS  (line 692, body d6b0db473344)
- [x] CCL_EQ  (line 685, body d2b9b2416375)
- [x] CCL_EXECUTE_BUF_SIZE  (line 2132, body 128351137a9c)
- [x] CCL_End  (line 322, body a8ec1c1966ef)
- [x] CCL_ExprSelfConst  (line 331, body d6b0db473344)
- [x] CCL_ExprSelfReg  (line 338, body add3101bdb16)
- [x] CCL_Extension  (line 415, body dbea8cd00789)
- [x] CCL_GE  (line 687, body 0be4bc9b0292)
- [x] CCL_GT  (line 684, body e91e357bd4f7)
- [x] CCL_HEADER_BUF_MAG  (line 63, body b6589fc6ab0d)
- [x] CCL_HEADER_EOF  (line 64, body 356a192b7913)
- [x] CCL_HEADER_MAIN  (line 65, body da4b9237bacc)
- [x] CCL_INVALID_CMD  (line 730, body c88e9804632f)
- [x] CCL_IterateMultipleMap  (line 475, body 9042d4228c01)
- [x] CCL_Jump  (line 126, body cc35a7395782)
- [x] CCL_JumpCond  (line 134, body 2fee0d789cea)
- [x] CCL_JumpCondExprConst  (line 361, body 8a4251cdadf6)
- [x] CCL_JumpCondExprReg  (line 374, body 2fa23e0955cf)
- [x] CCL_LE  (line 686, body c9e339801005)
- [x] CCL_LS  (line 683, body 9042d4228c01)
- [x] CCL_LSH  (line 678, body 432a3e71f41d)
- [x] CCL_LSH8  (line 680, body 35d9fb27f078)
- [x] CCL_LookupCharConstTbl  (line 663, body 0be4bc9b0292)
- [x] CCL_LookupIntConstTbl  (line 657, body c9e339801005)
- [x] CCL_MINUS  (line 671, body 2113f7bfab26)
- [x] CCL_MOD  (line 674, body cc35a7395782)
- [x] CCL_MUL  (line 672, body d6c81010612a)
- [x] CCL_MapMultiple  (line 566, body e91e357bd4f7)
- [x] CCL_MapSingle  (line 646, body d2b9b2416375)
- [x] CCL_NE  (line 688, body 52188a82c831)
- [x] CCL_OR  (line 676, body afccd2a6961e)
- [x] CCL_PLUS  (line 670, body 58783889d2d1)
- [x] CCL_READ_CHAR  (line 794, body af85a5f1f8f3)
- [x] CCL_RSH  (line 679, body 970d12842fd7)
- [x] CCL_RSH8  (line 681, body ecbaa4b841e0)
- [x] CCL_ReadBranch  (line 251, body 9042d4228c01)
- [x] CCL_ReadJump  (line 212, body c78b5e6fa049)
- [x] CCL_ReadJumpCondExprConst  (line 387, body 9b583d195d8b)
- [x] CCL_ReadJumpCondExprReg  (line 401, body 3d650c5ca203)
- [x] CCL_ReadMultibyteChar2  (line 433, body 58783889d2d1)
- [x] CCL_ReadRegister  (line 231, body 44dd34a91ebe)
- [x] CCL_SUCCESS  (line 696, body d1eabd2b1087)
- [x] CCL_SUSPEND  (line 707, body f0a2a3d95e63)
- [x] CCL_SetArray  (line 115, body 9fc6fa67f2a4)
- [x] CCL_SetConst  (line 107, body d6c81010612a)
- [x] CCL_SetExprConst  (line 347, body f331dcff7d8d)
- [x] CCL_SetExprReg  (line 355, body 11aee9e9e43d)
- [x] CCL_SetRegister  (line 95, body 58783889d2d1)
- [x] CCL_SetShortConst  (line 101, body 2113f7bfab26)
- [x] CCL_TranslateCharacter  (line 449, body d6c81010612a)
- [x] CCL_TranslateCharacterConstTbl  (line 458, body 9fc6fa67f2a4)
- [x] CCL_WRITE_CHAR  (line 762, body ca217c63dfef)
- [x] CCL_WRITE_STRING  (line 774, body 9e09b631ac18)
- [x] CCL_WriteArray  (line 311, body 52188a82c831)
- [x] CCL_WriteArrayReadJump  (line 195, body ecbaa4b841e0)
- [x] CCL_WriteConstJump  (line 161, body 432a3e71f41d)
- [x] CCL_WriteConstReadJump  (line 169, body 970d12842fd7)
- [x] CCL_WriteConstString  (line 296, body 0be4bc9b0292)
- [x] CCL_WriteExprConst  (line 240, body 3a6398cbb2aa)
- [x] CCL_WriteExprRegister  (line 278, body d2b9b2416375)
- [x] CCL_WriteMultibyteChar2  (line 440, body 2113f7bfab26)
- [x] CCL_WriteRegister  (line 265, body e91e357bd4f7)
- [x] CCL_WriteRegisterJump  (line 142, body afccd2a6961e)
- [x] CCL_WriteRegisterReadJump  (line 149, body 7b524218c065)
- [x] CCL_WriteStringJump  (line 182, body 35d9fb27f078)
- [x] CCL_XOR  (line 677, body 7b524218c065)
- [x] EXCMD  (line 932, body a198a91f7c92)
- [x] GET_CCL_CODE  (line 755, body cbda4496cbf6)
- [x] GET_CCL_RANGE  (line 745, body 1a0d361e54e0)
- [x] GET_HASH_TABLE  (line 48, body c8a87981bfe6)
- [x] IN_INT_RANGE  (line 758, body fb677876eb37)
- [x] MAX_MAP_SET_LEVEL  (line 579, body 22d200f8670d)
- [x] POP_MAPPING_STACK  (line 611, body 85cfd8d8c442)
- [x] PUSH_MAPPING_STACK  (line 594, body 1485d2846bf8)
- [x] RRR  (line 929, body 78f91296c1e3)
- [x] Rrr  (line 930, body bda0d72d26bc)
- [x] rrr  (line 928, body 73a9e5f2cc6b)

## Types and their fields — 1
- [x] ccl_prog_stack: Lisp_Object* ccl_prog, int ic, int eof_ic

## File-scope objects — 7
- [x] static Lisp_Object Vccl_program_table  (line 45, static)
- [x] static tr_stack mapping_stack  (line 587, static)
- [x] static tr_stack mapping_stack_pointer  (line 588, static)
- [x] static int stack_idx_of_map_multiple  (line 592, static)
- [x] int ccl_backtrace_table  (line 846)
- [x] int ccl_backtrace_idx  (line 847)
- [x] static struct ccl_prog_stack ccl_prog_stack_struct  (line 865, static)

## Decisions

### `#if` branches (PORTING.md 7.2, status 3.49)

`src/ccl.c` is **not** in `docs/baseline/config/cond-eval-verified.txt` (only `src/ccl.h` is), so
the frozen macro table is not consulted for it and the arms are *carried* rather than recorded as
dead.  `python3 tools/check-cond-eval.py src/ccl.c` reports
`7 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so clang and the evaluator agree on the
frozen configuration's branch selection; the arms are carried anyway because the *list*, not the
evaluator, is the declaration checker's authority.

1. **C 607-609** `#if 13 <= __GNUC__` / `# pragma GCC diagnostic ignored "-Wanalyzer-out-of-bounds"`
   / `#endif` — a constant condition around a pragma.  `__GNUC__` is 4 on the frozen target, so
   the guard is false and the pragma is not part of the frozen build; a pragma has no Rust
   spelling and the arm holds no code, so the text is recorded here (nothing in the ledger
   depends on it).
2. **C 718-738** `#ifndef CCL_DEBUG` / `#else` / `#endif`, the two `CCL_INVALID_CMD` definitions —
   a **structural** condition.  Both arms are carried inside `ccl_driver` (where the macro is
   defined), each behind `#[cfg (not (feature = "ccl_debug"))]` / `#[cfg (feature = "ccl_debug")]`.
   `CCL_DEBUG` is a source-level macro, not a `config.h` key, so they are not in
   `xtask dead-arms` and no configuration compiles the `ccl_debug` arm.
3. **C 844-855** `#ifdef CCL_DEBUG` (`CCL_DEBUG_BACKTRACE_LEN`, `ccl_backtrace_table`,
   `ccl_backtrace_idx`, `ccl_debug_hook`) — structural, carried behind
   `#[cfg (feature = "ccl_debug")]`; `ccl_debug_hook` is C's non-`static` definition, so it keeps
   `#[unsafe(no_mangle)]`.
4. **C 898-900** and **C 905-910** (inside `ccl_driver`) and **C 1805-1829** (inside the error
   handler) — the same condition, carried the same way.  C 1805-1829's text names `dst_bytes`,
   which `ccl.c` never declares (`grep -n dst_bytes emacs/src/ccl.c` finds only this arm and the
   `#if 0` one): upstream's own dead arm cannot compile.  It is carried **verbatim** because the
   checker compares the tokens of a blind file's arms (and the `memcpy` family's occurrence check
   needs both of its `memcpy` calls), and it is not compiled by anything — `CCL_DEBUG` is not a
   `config.h` key, so `dead-arms` never turns it on.  This is the one carried arm that is not
   compilable; the ledger records it rather than inventing a `dst_bytes` C does not have.
5. **C 1848-1861** `#if 0 … #else … #endif` in the error handler — `#if 0` is dead for the tool
   itself (`tools/cinventory.py` skips `#if 0` unconditionally), so the arm (the "copy the
   remaining source data" text, which also names `dst_bytes`) is recorded here, not transcribed;
   the live `#else` arm (`src = src_end;`) is in the unit.

### Bitfields, unions, function pointers, flexible arrays, alloca

- `struct ccl_program`'s two `bool_bf : 1` members (`last_block`, `quit_silently`) are one 1-byte
  storage unit at offset 84 in C (`bool_bf` is `bool` on this target, `conf_post.h` 56-61), so the
  mirror packs them into `bits: u8` with the `CCL_PROGRAM_LAST_BLOCK`/`CCL_PROGRAM_QUIT_SILENTLY`
  masks; the two `docs/ports/token-aliases.tsv` families (`last_block`, `quit_silently`) keep
  every read and write occurrence-checked (PORTING.md 1.3, P-80).
- **Measured layout** (`clang -std=gnu23`, the frozen target; the probe is the two structs plus
  `<stddef.h>`, printing `sizeof`/`offsetof`): `struct ccl_program` 88/align 8 with `idx`@0,
  `size`@8, `prog`@16, `ic`@24, `eof_ic`@28, `reg`@32, `status`@64, `buf_magnification`@68,
  `stack_idx`@72, `consumed`@76, `produced`@80, the two bits at 84:0-84:1 (the byte reads `0x01`
  then `0x03` as the two are set); `struct ccl_spec` 88 with `ccl`@0; `struct ccl_prog_stack` 16
  with `ccl_prog`@0, `ic`@8, `eof_ic`@12.  The mirror and the unit pin those numbers with
  `const _` assertions (14 for `ccl_program`, 2 for `ccl_spec`, 3 for `ccl_prog_stack`).
- No unions, no function-pointer-with-context fields, no flexible array members and no `alloca`
  in this file; no `setjmp`/`longjmp` and no computed `goto`.
- `struct ccl_prog_stack` and `tr_stack` are declared in the unit (C declares them in the .c), so
  `tools/gen-layout-fields.py`'s `STRUCTS` list does not name them; both are
  `#[derive (Clone, Copy)]`, which is exact for a plain C record (PORTING.md 0.1-9) and is what
  lets their `static` arrays be zero-initialised (`[X { … }; 256]`).

### Deviations from the reference

1. **`ccl_driver`'s five labels are the `ccl_goto` values.**  Rust has no label-as-value and a
   `match` arm cannot be jumped into, so the whole `for (;;)` body is one `'ccl_repeat: loop` that
   *leaves with* the label a C `goto` would reach (`break 'ccl_repeat ccl_goto::ccl_finish` for
   `goto ccl_finish`, `...ccl_error_handler` for `goto ccl_error_handler` and for the loop's own
   `break`, `continue 'ccl_repeat` for `goto ccl_repeat`), and the two labels *inside* the switch
   (`ccl_set_expr`, `ccl_expr_self`) are an entry stage: the arms that reach them set `tail` and
   the shared statements run once after the `match` (PORTING.md 2.2, the bytecode.rs/tparam.rs
   shape).  `none` is the "no shared tail" state.  C's trailing `break` after the switch is the
   loop body's `continue 'ccl_repeat`.
2. **Nine statement macros are defined inside `ccl_driver`** (`CCL_CALL_FOR_MAP_INSTRUCTION`,
   `CCL_SUCCESS`, `CCL_SUSPEND`, `CCL_INVALID_CMD` ×2, `CCL_WRITE_CHAR`, `CCL_WRITE_STRING`,
   `CCL_READ_CHAR`, `CCL_DECODE_CHAR`, `CCL_ENCODE_CHAR`) plus the five field aliases
   (`rrr`, `RRR`, `Rrr`, `ADDR`, `EXCMD`) and `GET_CCL_RANGE`/`GET_CCL_CODE`.  C defines the
   statement macros at file scope over this function's locals and jumps to its labels; a
   `macro_rules!` resolves local variables *and labels* at its **definition** site, so defining
   them after those locals and inside the label they jump to is what lets every call site below
   keep C's own shape (the bytecode.rs P-57 placement).  `GET_CCL_RANGE`/`GET_CCL_CODE` must be
   inside too: their bodies invoke `CCL_INVALID_CMD`, which is only in scope there (a macro call
   inside a macro body resolves where the outer macro is *defined*).  Their C comments stay at
   C's file-scope positions with a pointer note, and the macro block sits at the top of the loop
   body because C's first `GET_CCL_CODE` (C 913) precedes C's `#define` block (C 926).
3. **C's `#define rrr field2` and friends are macros that must be *textually* substituted**: a
   macro expansion is not a place, so C's four `rrr = 7` are `field2 = 7` — C's own preprocessed
   text (the bytecode.rs `TOP = x` precedent).  Reads keep the aliases (`rrr! ()`, `RRR! ()`,
   `Rrr! ()`, `ADDR! ()`, `EXCMD! ()`).
4. **C's pointer indexing is spelled out**: Rust raw pointers cannot be indexed, so `reg[rrr]` is
   `*reg.offset (rrr! () as isize)`, `ccl_prog[ic]` is `*ccl_prog.offset (ic as isize)`,
   `ccl_prog_stack_struct[i]` is `ccl_prog_stack_struct[i as usize]` (a static array is
   indexable), `mapping_stack_pointer->x` is `(*mapping_stack_pointer).x`, `src++`/`dst++` are the
   load/store followed by `offset (1)`, and `a - b` on pointers is `a.offset_from (b)`.  A Rust
   array's one-past-the-end address (`&mapping_stack[MAX_MAP_SET_LEVEL]`) is
   `mapping_stack.as_mut_ptr ().offset (MAX_MAP_SET_LEVEL as isize)`, because `&raw mut arr[N]`
   for `N == len` is a compile-time error.
5. **Uninitialised locals** (PORTING.md 1.5; each is named in a `/* PORTING.md 1.5: … */` comment
   in the unit): `code`'s companions `field1`/`field2`, `jump_address`, `j`, `op`, `charset`,
   `called_ccl`, `ccli`, `ncode`, `orig_op`, `eop`, `h`, `opl`, `map`, `content`, `attrib`,
   `value`, `point`, `size`, `map_vector_size`, `map_set_rest_length`, `fin_ic`, `msg`, `msglen`,
   `vp`, `slot`, `i`/`veclen`/`result`/`contents`/`val` (`resolve_symbol_ccl_program`), `val`/
   `slot` (`ccl_get_compiled_code`) and `dst_bytes` (the carried `ccl_debug` arm, which C does
   not declare at all) — the two that C *declares and then assigns in every path* (`src`/`src_size` in `Fccl_execute_on_string`) keep the declare-then-assign shape, which
   Rust allows there.
6. **`ckd_add`/`ckd_sub`/`ckd_mul`** are `overflowing_*` with C's "store the wrapped value"
   semantics: the reference's `<stdckdint.h>` is clang's (`__builtin_*_overflow`), which stores
   the low-order bits and returns the overflow flag (PORTING.md 1.1/P-15, the bignum.rs shape).
   The two `ckd_sub (&reg[rrr], 0, reg[rrr])` sites are spelled `c_int::overflowing_sub (0, x)`
   so C's `0` operand stays inside the family's call, which the occurrence check requires.
7. **`FALLTHROUGH`** (C 1019, 1226, 1237): the `match` arms that C sends on select their entry
   (`if code & 0x1F == CCL_ReadBranch { … }`), the shared statements appearing once, and the
   `goto have_prefix_arg`-style token is exempted in `docs/ports/macro-body-notes.tsv`.
8. **C's `#if 13 <= __GNUC__` pragma** (see above) has no Rust spelling; the file-scope macros
   that lost a token to Rust's syntax (`goto`, `switch`/`case`/`default`, `fallthrough`, the
   `do { … } while (0)`'s `0` and the DEFVARs' `doc`) are recorded in
   `docs/ports/macro-body-notes.tsv`, and the two bitfield families in
   `docs/ports/token-aliases.tsv`.
9. **`CCL_EXECUTE_BUF_SIZE`** (C 2132) is a `const` inside `Fccl_execute_on_string`, because C's
   `#define` is inside that function; the ledger sees the constant either way.
10. **`sprintf`'s `%x` arguments** are cast to `c_uint` (`code & 0x1Fu`, `code + 0u`) and the
    `%d` one stays `c_int`, as C's default argument promotions produce.
11. **`make_specified_string`'s fourth argument** is C's `NILP (unibyte_p)` (`bool`), and
    `outp - outbuf` is `outp.offset_from (outbuf)`.
12. **The `ccl.h` mirror landed with this unit** (`femacs/src/include/ccl.rs`, the composite.rs
    precedent): `struct ccl_program`, `struct ccl_spec`, the five `CCL_STAT_*` constants,
    `CODING_SPEC_CCL_PROGRAM`, `CHECK_CCL_PROGRAM` and the two prototypes in `ccl_decls` + glob
    import.  Landing it **deletes the `struct ccl_spec` stand-in `femacs/src/include/coding.rs`
    carried** for `coding.h` 447 and degrades that field to the ABI-equal `*mut c_void` with a
    `/// C:` note: PORTING.md P-70 puts the deletion in this same commit (two same-named
    definitions are one `E0428` in a unit that includes both mirrors) and P-72 ③ is the rule for
    a mirror that may only name a type it declares itself.  `docs/ports/include-ccl.md` records
    the mirror side; `coding.md`'s stand-in row is now stale and is the only place that mentions
    it (this unit does not edit another unit's ledger).
13. **`charset.h` has no mirror yet** (`docs/port-map.md` still lists
    `femacs/src/include/charset.rs` as unstarted), so the names this unit's live text takes from
    it are declared at the use site with C's own bodies (PORTING.md 0.1-2/P-72, the chartab.rs
    shape): `enum charset_method` and its four members, `enum charset_attr_index`'s `charset_id`
    and `charset_decoder`, `struct charset` (with the six `bool_bf` members packed into `bits`),
    the six `CHARSET_*_P` masks, `struct charset_table`, the `charset_table`/`charset_unicode`
    objects, `char_charset`/`decode_char`, `CHARSET_ATTR_ID`/`CHARSET_ATTR_DECODER`/
    `CHARSET_ATTRIBUTES`/`CHARSET_DECODER`/`CHARSET_ID`/`CHARSET_FROM_ID` and `DECODE_CHAR` (whose
    operands are bound once, because C's `->` on an `expr` fragment has no Rust spelling).  They
    move to the mirror when charset.c lands.
14. **Two lisp.h declarations the `lisp.rs` mirror does not carry yet** are declared at their use
    site and reported as mirror gaps: `larger_vector` (lisp.h 4245) and `make_specified_string`
    (lisp.h 4566).  `<intprops.h>`'s `INT_DIVIDE_OVERFLOW` has no mirror at all (P-26), so it is a
    unit-local macro (the canonicalize-lgpl.rs shape), and `<stdio.h>`'s `FILE`/`sprintf` come
    from the `stdio.rs` mirror.
15. **The `coding.rs` mirror's types needed `composite.rs` first** (`coding.h` 284 includes
    `<composite.h>`), `keyboard.rs` needed `termhooks.rs` (`keyboard.h` 24) and `lisp.rs`'s
    bignum chain needed `mini-gmp.rs`/`bignum.rs`; the unit's `include!` list carries the C
    includes plus those transitive mirrors, each with the C line that reaches it.
16. **Line count**: the port is 3217 lines against C's 2394 (34.4%), which
    `docs/ports/line-drift-tolerance.tsv` declares with the **`total`** basis (the carried arms
    keep the file's whole text) and the reason; the per-function token comparison is the real
    guard and it is `OK`.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the same
argument order; the port adds no call the reference does not make (the `charset_attributes_getter`
and `DECODE_CHAR` calls are C's own macros expanding at the same points, and the `sprintf`
varargs keep C's promoted argument types).
