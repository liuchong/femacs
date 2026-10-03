# emacs/src/bytecode.c (1852 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 2
- [x] byte-code  (Fbyte_code, 3/3, doc 7fadbc008e25, line 298)
- [x] internal-stack-stats  (Finternal_stack_stats, 0/0, doc 9f33a7c798af, line 426)

## Functions defined — 8
- [x] bcall0  (line 323, static)
- [x] init_bc_thread  (line 380)
- [x] free_bc_thread  (line 390)
- [x] mark_bytecode  (line 396)
- [x] valid_sp  (line 446, static)
- [x] exec_byte_code  (line 481)
- [x] get_byte_code_arity  (line 1810)
- [x] syms_of_bytecode  (line 1823)

## Macros defined — 18
- [x] BC_REG_PC  (line 463, body da39a3ee5e6b)
- [x] BC_REG_TOP  (line 462, body da39a3ee5e6b)
- [x] BC_STACK_SIZE  (line 337, body 1c56e48487fa)
- [x] BYTE_CODES  (line 73, body 0f46127845f8)
- [x] BYTE_CODE_SAFE  (line 31, body 7cb6efb98ba5)
- [x] BYTE_CODE_THREADED  (line 43, body da39a3ee5e6b)
- [x] CASE  (line 577, body b5994175ed2d)
- [x] CASE_ABORT  (line 593, body df66dd56cc61)
- [x] CASE_DEFAULT  (line 591, body da39a3ee5e6b)
- [x] DEFINE  (line 611, body 62990c648c03)
- [x] DISCARD  (line 291, body 8fa27179598c)
- [x] FETCH  (line 273, body ab105fc6a289)
- [x] FETCH2  (line 278, body d111a9f663bc)
- [x] FIRST  (line 585, body 7cff8a53c78a)
- [x] NEXT  (line 581, body ee25a1e8a1be)
- [x] POP  (line 287, body 00ec3b274283)
- [x] PUSH  (line 283, body 175cd268371d)
- [x] TOP  (line 296, body 2b98294da9da)

## Types and their fields — 1
- [x] bc_frame: struct bc_frame* saved_fp, Lisp_Object* saved_top, const unsigned char* saved_pc, Lisp_Object fun, Lisp_Object next_stack

## File-scope objects — 0

## Decisions

`emacs/src/bytecode.c` @ `fd7afced403`; port `femacs/src/bytecode.rs` (1931 lines vs C's 1852,
**1773** live — the 8.9% drift against the live baseline is declared in
`docs/ports/line-drift-tolerance.tsv` with the `live` basis, because the 80 dead lines are
recorded here instead of carried).  The gates are
`build-tu src/bytecode.c` ->
`built src/bytecode.rs -> ../build/femacs/bytecode.o`,
`python3 tools/extract-abi.py --check emacs/src/bytecode.c femacs/src/bytecode.rs` -> `OK`,
`python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt` ->
`78 units compared, 0 mismatched, 1 still pending` (the compared set is the `lib/` units with
a reference object; it is not this port's) and `python3 tools/check-globals.py` ->
`240 units, 0 DEFVAR_* variables without a definition in their own unit`.  `src/bytecode.c` is
in `docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it: there is
no 3.31 exemption here and the full comparison applies.

### The computed-goto dispatcher (PORTING.md 2.3, the decision this file is about)

The frozen configuration **does** define `BYTE_CODE_THREADED`: the macro table has `__GNUC__`
4 and no `__STRICT_ANSI__`, `BYTE_CODE_METER` is undefined and `BYTE_CODE_SAFE` is `false`, so
C 41-44's condition holds.  C's live interpreter is therefore the threaded one — `CASE(OP)` is
the label `insn_ ## OP`, `NEXT` is `goto *(targets[op = FETCH])`, and C 603-616's dispatch
table routes `[0 … Bconstant-1]` to `insn_default` and `[Bconstant … 255]` to
`insn_Bconstant`.  Rust has no label-as-value (`&&label`), so

- `BYTE_CODE_THREADED` is a **structural condition** (P-03, PORTING.md §9.2) and dead;
  `docs/ports/excluded-macros.txt` records it (kind=config), together with `BC_REG_TOP` /
  `BC_REG_PC` (C 455-464's `register … asm ("rbx")` / `("r12")` spellings, empty on this
  target's `#else` arm).
- the port transcribes C's **switch-shaped arm** (C 594-600: `CASE(OP)` is `case OP`, `NEXT` is
  `break`, `FIRST` is `switch (op)`, `CASE_DEFAULT` is `case 255: default:`, `CASE_ABORT` is
  `case 0`).  `CASE`/`NEXT`/`FIRST`/`CASE_DEFAULT`/`CASE_ABORT` keep C's names and invocation
  shape; `FIRST! (op, { … })` takes C's switch body as its last argument and expands to
  `match op { … }`, `NEXT! ()` is the dispatch loop's back edge, and C's stacked labels
  (`CASE (X): CASE (Y):`) are OR-patterns.
- **This is a semantic choice, not just a spelling one**, and it is P-03's: an opcode C's
  switch-shaped arm sends to `default` (the `Bconstant` arm, which pushes
  `vectorp[op - Bconstant]`) is one the *threaded* arm would send to `insn_default` (the
  `CASE_ABORT` error arm).  The port reproduces the switch-shaped arm's behaviour, which is
  what C's own `BYTE_CODE_SAFE` bound check exists to catch.  The tokens C's live threaded text
  has and this text cannot spell (`goto`, `targets`, `insn_default`, `insn_bconstant`,
  `byte_codes`, `255`, `256`, `bc_reg_top`, `bc_reg_pc`) are one row of
  `docs/ports/macro-body-notes.tsv`, and the four dispatcher macros have their own rows.

### C's labels: the `goto` state and the shared tails

The nine labels (`setup_frame`, `varref`, `varset`, `varbind`, `docall`, `dounbind`,
`op_branch`, `pushhandler`, `exit`) have no Rust counterpart (PORTING.md 2.2), so:

- `op`/`arg`/`type` (C 549-551's declarations inside the `while (true)` body) are hoisted above
  the dispatch loop, because a tail runs at the head of the *next* iteration and reads them.
- the eight in-switch tails are the arms of a `match target` at the head of the dispatch loop
  (`if goto != NONE`), in C's order: `VARREF`, `VARSET`, `VARBIND`, `DOCALL`, `DOUNBIND`,
  `EXIT`, `OP_BRANCH`, `PUSHHANDLER`.  The constants carry C's label text, so the token
  comparison still sees `varref`, `varset`, `varbind`, `docall`, `dounbind`, `op_branch`,
  `pushhandler` and `exit` (case folding is what makes `VARREF` count for `varref`).
- each `goto L` is `goto = L;`; where C's code after the jump would otherwise run (the
  conditional jumps), the next statement is `NEXT! ()`, which is exactly C's jump (the head
  consumes the pending label).
- `setup_frame:` is the outer loop, so the `docall` fast path's `goto setup_frame` is
  `continue 'setup_frame` and re-runs the setup exactly as C's backward jump does
  (C re-executes the declarations after the label).
- `exit:` is the `EXIT` arm, which breaks the dispatch loop; C's `exit:` statements then run
  where C has them.
- `Bswitch`'s two `goto op_branch`s leave the table search, so the two sites set `goto` and
  `break` out of the search loop; the tail then runs at the head, outside the loop.
- `op_branch`'s `quitcounter` and the `PUSHHANDLER` tail's `bc = &current_thread->bc;` are
  assignments to the outer locals C assigns (the port shadows only the names C shadows).

### Dead text (not transcribed)

80 of the file's 1852 lines are dead and are recorded here instead of transcribed (P-08;
bytecode.c is a `src/` file, so no dead text is kept as `#[cfg (feature = …)]`):

- C 47-67: the `BYTE_CODE_METER` macro block (`METER_2`/`METER_1`/`METER_CODE`).
- C 455-460 and 469-471: the `__x86_64__`/`__aarch64__` `BC_REG_TOP`/`BC_REG_PC` arms and the
  two `#pragma GCC diagnostic` lines (both are `__GNUC__ && !__clang__` arms).
- C 484-485, 560-565 and 768-779: the meter arms inside `exec_byte_code` (`this_op`,
  `prev_op`, `METER_CODE`, and the `byte-metering-on`/`get`/`put` histogram).
- C 594-600: the switch-shaped `#define`s.  The port transcribes their *meaning* (P-03,
  above); the text itself is not the frozen build's.
- C 1803-1804: the `#pragma GCC diagnostic pop` pair.
- C 1830-1850: `syms_of_bytecode`'s meter arm (`Vbyte_code_meter`, `byte_metering_on`,
  `Qbyte_code_meter`); it is also why the ledger's "File-scope objects" is 0.

Two **live** regions are also not carried, and they are the P-03 decision rather than dead
text: C 574-593 (the threaded `CASE`/`NEXT`/`FIRST`/`CASE_DEFAULT`/`CASE_ABORT` definitions)
and C 603-616 (the `targets` dispatch table, whose one `DEFINE` line is a `#define`).
`src/bytecode.c` is in `docs/baseline/config/cond-eval-verified.txt`, and
`python3 tools/check-cond-eval.py src/bytecode.c` reports
`14 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the branch selection above is the
clang-verified one; no `source-header-macros.tsv` row was needed.

### Deviations recorded

1. **The dispatcher** (the section above): `CASE`/`NEXT`/`FIRST`/`CASE_DEFAULT`/`CASE_ABORT`
   are defined once, before the dispatch loop (C defines them inside the `while` body, and the
   port's head tails use `NEXT`, so P-57's definition-before-use order fixes the place).
   `NEXT` is `continue` where the switch arm's is `break` — with the whole switch inside the
   loop, `continue` *is* C's back edge (a Rust `break` would leave the loop).
2. **`TOP` is C's lvalue `(*top)`.**  A macro expansion is not a place, so C's `TOP = x` is
   `*top = x` (about forty sites) and reads keep `TOP! ()`.  C's `&TOP` is `top` (the address
   of the top cell), so `Flist (n, &TOP)` and `Fconcat (2, &TOP)` are `Flist (n, top)` and
   `Fconcat (2, top)`.
3. **The six dispatcher macros are defined inside `exec_byte_code`.**  C 271-297 defines
   `FETCH`/`FETCH2`/`PUSH`/`POP`/`DISCARD`/`TOP` at file scope over `exec_byte_code`'s
   `top`/`pc` locals; `macro_rules!` hygiene resolves identifiers at the *definition* site, so
   the port defines them after those locals (the first use is the argument push).  `FETCH` and
   `FETCH2` yield `ptrdiff_t` (C's `unsigned char` promotes to `int`/`ptrdiff_t` at every use).
4. **The byte-code list is a macro plus constants.**  C's
   `enum byte_code_op { #define DEFINE(name, value) name = value, BYTE_CODES #undef DEFINE };`
   has no Rust counterpart (an X-macro in an enum body), so `DEFINE!` emits C's
   `name = value` as file-scope constants and `BYTE_CODES! ()` is the same list at the same
   place; the enum tag names nothing at its use sites.  The 168 names and values were
   compared against C's list mechanically.  The members are `ptrdiff_t` because every C use
   converts them to `ptrdiff_t` (`arg = op - Bvarref`, the `match` arms,
   `vectorp[op - Bconstant]`).  `tools/cinventory.py`'s enum scanner reads the bare
   `BYTE_CODES` line as an enum member of value 0 (the `#define DEFINE` line inside the enum
   body defeats it), so the unit also defines `pub const BYTE_CODES: ptrdiff_t = 0` with that
   comment; nothing reads it, and the macro's body is still the guard.
5. **`bc->fp` is untyped in the mirror.**  `struct bc_thread_state.fp` is `*mut c_void` in
   `femacs/src/include/thread.rs` (P-55, the type no mirror carries), so each use casts
   (`(*bc).fp as *mut bc_frame`).  `struct bc_frame` itself is defined here as C defines it,
   with the flexible array member as `[Lisp_Object; 0]` (§1.3).  Its layout is pinned by six
   `const _` assertions whose numbers come from the C compiler (see "Measured layout" below);
   the type is declared in this unit rather than in a mirror, so
   `tools/gen-layout-fields.py`'s `STRUCTS` list does not name it.
6. **Uninitialized locals** (PORTING.md 1.5): `op`/`arg`/`type_` are hoisted and
   zero-initialized; `res` (`Bdiff`/`Bplus`/`Bmult`/`Bquo`), `size` (`Baref`/`Baset`) and the
   `VARREF` tail's `v2` are declared before C's first assignment; the `DOCALL` tail's `val`
   keeps C's declare-then-assign shape (`let mut val: Lisp_Object;` assigned in both branches).
7. **`ckd_mul`** (`Bmult`) is `overflowing_mul` with "write the target only when there is no
   overflow" (P-15) inside C's single condition; the fixnum fast paths use
   `wrapping_add`/`wrapping_sub` (PORTING.md 1.1) and compare with `FIXNUM_OVERFLOW_P`; `Bquo`
   and `Brem` keep C's `/` and `%` behind C's `!= 0` check.
8. **Bool-to-int conversions Rust will not insert**: `quitcounter += new_pc < pc;` is
   `wrapping_add ((new_pc < pc) as c_uchar)`, `quitcounter = (unsigned char) -1;` is
   `(-1 as c_int) as c_uchar`, `int mandatory = args_template & 127` is
   `(args_template & 127) as c_int`, `nargs - rest` is `nargs - if rest { 1 } else { 0 }`,
   the `? Qt : Qnil` idioms and `rest ? Qmany : make_fixnum (nonrest)` and
   `FUNCTIONP (handler) ? bcall0 : prog_ignore` are `if`/`else` (over `Some (…)` for the two
   function pointers, PORTING.md 1.4).
9. **`++`/`--`**: C's `++lisp_eval_depth`, `specpdl_ptr--`, the `i++`/`args++` of the argument
   push and the `i--` of the search loops are the two-statement or `while` forms (the dired.rs
   precedent), keeping C's order of test/update/body.
10. **`lisp_eval_depth` is `i64` in the mirror** where C's is `EMACS_INT`, so the two
    comparisons against `max_lisp_eval_depth` carry `as EMACS_INT`.
11. **`record_in_backtrace`'s return value**: `femacs/src/include/lisp.rs` drops it (the
    mirror returns `()`), so the port takes C's `count` with `SPECPDL_INDEX ()` at the same
    point — C 3760-3766's `specpdl_ref count = SPECPDL_INDEX ()`.  This is a **mirror gap for
    the integrator**: the Lisp entry's own `count1` is what `do_debug_on_call` needs, and
    another unit reaching for it will hit the same wall.  No mirror was edited.
12. **`sys_setjmp (c->jmp)`**: thread.h's `sys_jmp_buf` is its own record (192 bytes,
    alignment 4, where lisp.rs's `jmp_buf` is the opaque `[u8; 192]`), so the port spells the
    cast the mirror's note asks for (`&raw mut (*c).jmp as *mut jmp_buf`).
13. **`%"pD"d`** (C 1692): `pD` is `"t"` (lisp.h 150), so the port's literal is
    `"Invalid byte opcode: op=%d, ptr=%td"` (the region-cache.rs precedent).  The `pD`
    token is two characters and the body check skips those.
14. **Forward declarations**: none in this file (C declares nothing ahead).  **`staticpro`**:
    none.  **`alloca`**: none.  **Bitfields**: the `u.s.redirect`/`u.s.trapped_write` reads of
    the `VARREF`/`VARSET` tails are the mirror's `u.s.bits` masks (lisp.rs 221-228, PORTING.md
    1.3); those two tokens are in the `exec_byte_code` notes row (the data.c rows' treatment).

### Measured layout

`struct bc_frame` (C 361-370) is declared in this unit, so the C compiler was asked directly
(the struct's own four-pointer declaration plus `_Static_assert`s, `clang -std=gnu23`):
`sizeof (struct bc_frame) == 32`, `saved_fp@0`, `saved_top@8`, `saved_pc@16`, `fun@24`,
`next_stack@32`.  The port's six `const _` assertions (right below the definition) pin those
same numbers, and `next_stack` being at 32 is what makes
`(char *) fp->next_stack` C 519's stack-overflow bound.

### Macro-body notes

`docs/ports/macro-body-notes.tsv` carries five rows for this unit: `CASE` (`insn`), `NEXT`
(`fetch,goto,op,targets`), `FIRST` (`next`), `CASE_ABORT` (`bstack_ref,case,default`) — each
the threaded definition whose text the switch-shaped port cannot spell (P-03) — and
`exec_byte_code` (`255,256,bc_reg_pc,bc_reg_top,byte_codes,insn_bconstant,insn_default,redirect,targets,trapped_write`),
which covers the dispatch table, the register spellings and the two packed symbol bitfields.
`docs/ports/excluded-macros.txt` carries `BYTE_CODE_THREADED`, `BC_REG_TOP` and `BC_REG_PC`.
`CASE_DEFAULT`'s body is empty in C and `_` here (the default arm Rust's `match` needs); no
note was needed for it.

### Condition table

Every `#if` in this file is decided by `docs/baseline/config/darwin-arm64.macros.tsv`:
`__GNUC__` 4, `__clang__` 1, `__STRICT_ANSI__` undef, `BYTE_CODE_SAFE`/`BYTE_CODE_METER`
undefined (the first by the file's own `#ifndef`), and `BYTE_CODE_THREADED` defined by the
file's own `#define`.  `src/bytecode.c` is in `cond-eval-verified.txt` and
`python3 tools/check-cond-eval.py src/bytecode.c` reports
`14 conditions, 0 unknown, 0 disagreements [VERIFIED]`.

### Line count

`docs/ports/line-drift-tolerance.tsv`'s `bytecode` row declares the `live` basis with a 0.10
tolerance: the 80 dead lines are recorded here rather than carried, so the denominator is
1773, and the port's 1912 (+7.8%) is the dispatcher's shape (P-03) — an arm per case with its
`{ … }` and `NEXT! ();`, the eight tails at the loop head, `*top = x` for every `TOP = x`,
`goto = L;` for every `goto L` — plus the unit's fixed costs (the `// femacs:` note, the
`#![allow]` block, the include list, the fourteen use-site declarations, the nine
label constants and the six `bc_frame` assertions).

### Changes outside the unit (for integrator review)

1. `docs/ports/excluded-macros.txt`: three rows (`BYTE_CODE_THREADED` kind=config, `BC_REG_TOP`
   / `BC_REG_PC` kind=syntax) — the P-03 branch and the two register spellings.
2. `docs/ports/macro-body-notes.tsv`: the five rows above.
3. `docs/ports/line-drift-tolerance.tsv`: the `bytecode` row (0.10, `live`).
4. Nothing else: no mirror, no `xtask`, no baseline file was edited.  The one mirror gap found
   (`record_in_backtrace`'s dropped return value) is recorded in deviation 11 rather than
   fixed here.
