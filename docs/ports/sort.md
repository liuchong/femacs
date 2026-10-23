# emacs/src/sort.c (1178 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 30
- [x] reverse_slice  (line 39, static)
- [x] sortslice_copy  (line 71, static)
- [x] sortslice_copy_incr  (line 79, static)
- [x] sortslice_copy_decr  (line 87, static)
- [x] sortslice_memcpy  (line 96, static)
- [x] sortslice_memmove  (line 105, static)
- [x] sortslice_advance  (line 114, static)
- [x] order_pred_lisp  (line 199, static)
- [x] order_pred_valuelt  (line 205, static)
- [x] inorder  (line 212, static)
- [x] binarysort  (line 224, static)
- [x] count_run  (line 273, static)
- [x] gallop_left  (line 327, static)
- [x] gallop_right  (line 409, static)
- [x] merge_init  (line 484, static)
- [x] merge_markmem  (line 530, static)
- [x] cleanup_mem  (line 553, static)
- [x] merge_register_cleanup  (line 591, static)
- [x] merge_getmem  (line 604, static)
- [x] needmem  (line 632, static)
- [x] merge_lo  (line 645, static)
- [x] merge_hi  (line 768, static)
- [x] merge_at  (line 892, static)
- [x] powerloop  (line 943, static)
- [x] found_new_run  (line 989, static)
- [x] merge_force_collapse  (line 1013, static)
- [x] merge_compute_minrun  (line 1037, static)
- [x] reverse_sortslice  (line 1053, static)
- [x] resolve_fun  (line 1061, static)
- [x] tim_sort  (line 1082)

## Macros defined — 3
- [x] GALLOP_WIN_MIN  (line 132, body 902ba3cda188)
- [x] MAX_MERGE_PENDING  (line 127, body 073be8aab6ae)
- [x] MERGESTATE_TEMP_SIZE  (line 137, body dd7c1a3d9d56)

## Types and their fields — 2
- [x] reloc: sortslice* src, sortslice* dst, ptrdiff_t* size, int order
- [x] stretch: sortslice base, ptrdiff_t len, int power

## File-scope objects — 0

## Decisions

Reference `emacs/src/sort.c` @ `fd7afced403`; port `src/sort.rs`.

### `#if` branches

None: `sort.c` has no conditional compilation (verified with `grep -n '#if'`).

### Bitfields, unions, function pointers, flexible arrays, `alloca`

- No bitfields, no unions, no flexible array members, no `alloca` in this file.
- `struct merge_state::pred_fun` is a C function pointer with context
  (`bool (*)(struct merge_state *, Lisp_Object, Lisp_Object)`); the port keeps the
  same signature as `Option<unsafe extern "C" fn (…) -> bool>` (PORTING.md 1.4) and
  calls it through `unwrap_unchecked`, whose precondition (non-NULL) is exactly the
  precondition C's indirect call has.
- `sortslice`, `stretch`, `reloc`, `merge_state` are `#[repr(C)]` with the C field
  order; `struct reloc`'s three pointers are stored as raw pointers, as in C, so the
  cleanup path can dereference them after the locals they point to were mutated.

### Deviations from the reference

1. **`static void merge_register_cleanup (merge_state *ms);` (C 481) has no Rust
   counterpart.** A Rust declaration plus a definition of one name in one module is
   `E0428`, and items are order-independent, so the definition at its C position is
   enough.  Recorded as the general rule in PORTING.md 0.3.
2. **`sortslice lo;` (C 1125) is zero-initialised** as
   `sortslice { keys: null_mut(), values: null_mut() }`: Rust rejects C's
   "declare, then assign every field on every branch" form (`E0381`), and the value
   written is the same one both branches overwrite.  General rule in PORTING.md 0.3.
3. **`goto Succeed` / `goto CopyB` (merge_lo) and `goto Succeed` / `goto CopyA`
   (merge_hi)** become the two labels of one state machine per function
   (`'flow: { … break 'flow Label::X; … }` then one `if let` for each tail), so no
   tail is skipped and no jump becomes an early `return` (PORTING.md 2.2).  Statement
   order inside the bodies is unchanged.
4. **`keys = allocated_keys = xzalloc (…)` (C 1132)** is split into two statements on
   one line (`keys = …; allocated_keys = keys;`), because Rust assignment is not an
   expression.  Same order, same single allocation.
5. **`memcpy` / `memmove`** become `ptr::copy_nonoverlapping` / `ptr::copy` with the
   element count, i.e. `n * sizeof (Lisp_Object)` bytes exactly as C copies.
6. **`NULL`** is `ptr::null_mut ()`; `eassume (p)` on a pointer is
   `eassume! (!p.is_null ())`, and comparisons that mix `Lisp_Object *` with
   `const Lisp_Object *` cast the const side (`hi as *mut Lisp_Object`), which is what
   C's implicit qualification change does.
7. **`bool` assignments of `0` / `1`** are `false` / `true`; `xmalloc` / `xfree` take
   `*mut c_void`, so the two Lisp arrays are cast at the call, as C's implicit
   `void *` conversion does.
8. **`static_assert (NIL_IS_ZERO)`** is `static_assert! (NIL_IS_ZERO)`: the C23
   keyword has no Rust spelling; the macro is defined once in `lisp.rs` (PORTING.md
   2.5).
9. **`PTRDIFF_WIDTH`** (C 127, from gnulib `<stdint.h>`) comes from the new
   `lib/include/stdint.in.rs` mirror (with `lib/include/limits.in.rs`
   for `_GL_INTEGER_WIDTH`); both are WIP mirrors marked as such.
10. **`calln (ms->predicate, a, b)`** is `calln! (…)`; the macro's `ARRAYELTS`
    replacement had to stop evaluating its argument (a constant expression cannot
    read a local); see the fix in `lisp.rs::CALLN!`.
11. `tim_sort` is the only external entry point (`#[unsafe(no_mangle)]
    pub unsafe extern "C" fn tim_sort`); every other function stays file-local
    (C's `static`).

### Call-relationship differences (D7)

None.  The call graph is unchanged: `tim_sort` → `merge_init`/`count_run`/
`binarysort`/`found_new_run`/`merge_force_collapse`/`unbind_to`, the merge helpers →
`gallop_left`/`gallop_right`, and the two predicates are still reached through the
`pred_fun` pointer.  Cross-file calls are the C ones: `Fvaluelt`, `mark_objects`,
`indirect_function`, `xmalloc`, `xzalloc`, `xfree`, `record_unwind_protect_ptr_mark`,
`unbind_to`, `SPECPDL_INDEX`, `make_invalid_specpdl_ref`, `specpdl_ref_valid_p`,
`word_size`, `calln` and the `Qnil`/`Qt`/… symbols.

## Adversarial review (2 reviewers, 2026-10-02)

Reviewer A (algorithm fidelity; differential harness, C vs Rust, 2,184 cases incl. 96
mid-sort signals) and reviewer B (toolchain/ABI/gate plumbing).  Resolutions:

| # | Finding | Resolution |
| --- | --- | --- |
| A1 P0 | `lisp.rs::XSYMBOL` returned the raw `XUNTAG` offset instead of C's `XBARE_SYMBOL (a)` (missing `+ lispsym`), so `resolve_fun` dereferenced a wild pointer — the file could not sort | Fixed in the mirror (`XSYMBOL` now returns `XBARE_SYMBOL (a)`); re-verified with reviewer A's probes (`XSYMBOL == XBARE_SYMBOL ? true`, crash probe exits 0) and the `--cfg enable_checking` build |
| A2 | `eassert!`/`eassume!` did not compile under `--cfg enable_checking` (mutable-static read in safe fns; two `eassert` sites dereferencing a raw pointer) | Macros read the flag in an `unsafe` block; the two accessor sites wrap the argument; `rustc --cfg enable_checking src/sort.rs` now compiles |
| A3 | `order_pred_valuelt` carried a statement C does not have (`let _ = ms;`) | Deleted |
| A4 | `static_assert (NIL_IS_ZERO)` was vacuous because `NIL_IS_ZERO` was hand-written `true` | `globals.rs` now also emits globals.h's `#define iQfoo N` as `pub const iQfoo: usize`; `lisp.rs` derives `NIL_IS_ZERO = iQnil == 0 && Lisp_Symbol == 0` |
| A5 | Ledger debt: the four findings and the fact that a *shared mirror* defect was the blocker | Recorded here and in `docs/spikes/2026-10-02-first-tu-sort.md`; behavioural gate registered as P-30 |
| B1 P1 | Crate-root subr prototypes in `lisp.rs` (`Fcons`/`Ffuncall`/`Fsignal`) collide with a TU's definition (`E0428`) | Deleted (their prototypes come from the generated `subrs` module); `Fcons`-defining probe compiles, `--check lisp.h` still `OK` |
| B3 P1 | `CALLMANY!` discarded `$f` (169 non-`Ffuncall` sites would call the wrong function) | `($f) (…)`; `calln!` passes the `Ffuncall` path; probe shows `_Fvector` referenced |
| B4 P1 | The checker change opened a stub bypass | Stub detection sees the unit plus its includes again; only names the C file itself defines are reported; `evil_tu.rs` reproduces `FAIL` |
| B5 | Duplicate subr names: last-wins lost the 8th `noreturn` | Generator merges by name, errors on differing signatures, ORs flags; 8 `-> !` |
| B6 | Prototype set is a whole-configuration superset (304 GUI-only names) | Recorded as deviation P-29 |
| B7 | `static_assert!` accepted only one argument | Accepts the optional C23 message |
| B8 | `_Noreturn` on the declaration only; `ATTRIBUTE_COLD`/`NO_INLINE` unrecorded | `DEFUN!` definitions must also be `-> !` (§5.1/§5.2); other attributes recorded as not expressible (P-25) |

Reviewer A's positive verification (independent of the fixes): 2,088 non-signal cases and
96 signal cases byte-identical to the C (including `cleanup_mem`'s relocation and the GC
mark traces), all 97 C comments verbatim, constants equal, drift +3.82 %.
