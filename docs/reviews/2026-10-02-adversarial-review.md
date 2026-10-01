# Adversarial review — 2026-10-02

Two adversarial reviewers worked on independent context windows. They were told
the artifact is wrong and asked to prove it. Neither edited the tree; both wrote
their scratch work under `/tmp`. This file records what they found, what was
fixed in the same turn, and what remains open.

Scope of the review: `PORTING.md` (reviewer A), and `tools/extract-abi.py`,
`tools/gen-port-map.py`, `tools/gen-lifetimes.py`, `tools/gen-baseline.py` with
their generated artifacts (reviewer B).

## Process finding (both reviewers)

The artifacts were being edited while the review ran, so the two reviewers saw
different hashes and reviewer B had to pin md5 snapshots mid-flight. Rule adopted
from now on: **freeze before review** — commit the artifacts, hand the reviewer
the commit, and review the committed state. Hash manifest: run
`shasum -a 256 tools/*.py PORTING.md LIFETIMES.tsv docs/port-map.md
docs/baseline/*.tsv` and record it in the review entry.

## Reviewer B — tooling and generated data (all P0s fixed this turn)

| # | Finding | Evidence | Fix |
| --- | --- | --- | --- |
| B-P0-1 | `--check` accepted a file whose every function body was `unimplemented!()`: it compared name sets and line counts only | stub file of `bytecode.c` reported `OK` | rewritten: `--check` now also compares subr signatures (Lisp name, min/max/n, docstring digest), macro body digests, constant values, struct field order, file-scope objects, rejects stub/empty bodies, and tightens line drift to 5% |
| B-P0-2 | `LIFETIMES.tsv` was structurally corrupt: 219 lines of raw C source injected by a multi-line global regex | TSV rows beyond 3707 contained whole DEFUN bodies | `tools/gen-lifetimes.py` rewritten on top of `tools/cinventory.py`; no multi-line regexes, every field flattened; malformed rows now 0 |
| B-P0-3 | Field extraction lost tab-indented fields, comma declarator lists, `ENUM_BF (T) … : 2` bitfields, and dropped the pointer `*` entirely | `struct Lisp_Symbol` missing 10 fields; `struct frame` missing 21; `unsigned char *data` recorded as `unsigned char data` | new field parser: pointer depth, arrays, bitfields, declarator lists; `struct Lisp_Symbol` 10/10, `frame` complete, `Lisp_String.data` now `pointer_depth=1` |
| B-P0-4 | `extract-abi.py` missed all-caps real functions (`CAR`, `XPNTR`, `ADVANCE`) and the `(XLI)` definition form, and mistook `Copyright (C)`, docstring prose and `static_assert` for functions | `lisp.h` reported 121 functions against 249 real | comment/string stripping plus a definition rule (`NAME (...)` followed by `{`); `lisp.h` now reports 268, `XLI/XIL/XLP/XTYPE/TAGGEDP/CHECK_TYPE` all present, `Copyright` gone |
| B-P0-5 | 378 phantom globals (17%) and phantom fields from comment prose | `buffer.h:651` comment parsed as a declaration | comment stripping in the shared extractor; globals are now only reported at file scope outside function bodies |
| B-P0-6 | `#if 0` dead branches contributed live macros/fields | `w32term.h:71` `refcount`, `charset.h:408` macros | `#if 0` / `#if false` tracking in `cinventory.dead_branch_map`; `#else`/`#elif` are treated as live (unknown configuration), which is the conservative choice |
| B-P0-7 | `gen-port-map.py` produced 44 colliding target paths (`.c` and `.h` of the same name in `lib/`, `lib-src/`, `exec/`, … mapped to one `.rs`) | 590 sources → 546 unique targets | headers in every C directory now map to `<dir>/include/NAME.rs`; 590 sources → 649 unique targets, 0 collisions |
| B-P0-8 | `docs/baseline/subrs.tsv` had a ghost subr `Ftestme` from a docstring, and the three subr totals disagreed (1,850 / 1,847 / 1,849) | `alloc.c:5047` docstring | single extractor for all baselines; the authoritative count is now **1,841 live DEFUNs** (1,849 raw occurrences − 8 inside `#if 0`), 1,736 unique C names, no ghost rows |
| B-P1 | `gen-baseline.py` covered only `src/` (250 of 590 files) and its totals omitted externs/globals | 340 sources had no inventory row | baselines now cover all 8 C directories: 590 files, 678,320 lines, 10,012 functions, 7,156 macros, 5,441 struct fields, 1,934 file-scope objects |

## Reviewer A — `PORTING.md` (blocking items fixed, rest tracked)

Blocking findings and their resolution in `PORTING.md` v2:

1. **Compile model would produce duplicate symbols.** `include!`-ing a header
   mirror containing `#[no_mangle]` definitions or `extern`-as-definition into 250
   compilation units gives `ld: duplicate symbols` (reviewer reproduced it). Now:
   header mirrors hold declarations only (`unsafe extern "C" { … }`), definitions
   live in exactly one file, `xtask` flattens the include graph (deduplicating
   what C did with include guards), and macros have a single `#[macro_export]`
   source.
2. **`defun!` docstrings were only comments.** Rust's lexer drops comments, so
   `stringify!` returned `doc : ,`; the `etc/DOC` byte-equality requirement had no
   mechanical support. Now: docstrings are Rust string literals inside `defun!`,
   and `--check` compares a docstring digest per subr.
3. **`eassert`/`eassume` semantics were written backwards.** In release builds C
   does not evaluate the expression at all, and in checking builds it is short
   circuited by the runtime `suppress_checking` flag. Now specified in three
   forms, with "release must not evaluate".
4. **Conservative stack scan was under-specified.** `__builtin_unwind_init`,
   `SET_STACK_TOP_ADDRESS` (two implementations), `test_setjmp` self-check,
   `GC_SETJMP_WORKS`, `GC_MARK_SECONDARY_STACK`, `NEAR_STACK_TOP` and the
   anti-tail-call `__asm__("")` device were all missing. Now a dedicated section,
   and `black_box`/empty `asm!` are explicitly rejected as substitutes.
5. **`alloca` strategy broke `sa_avail`.** The C budget is 16 KB shared per
   function, and loop-accumulating call sites rely on distinct addresses. Now the
   port keeps the budget accounting, covers `SAFE_ALLOCA_LISP`,
   `SAFE_ALLOCA_STRING` and `REGEX_USE_SAFE_ALLOCA`, and lists the loop sites.
6. **Computed goto**: Rust has no label-as-value, so the threaded interpreter is
   not portable; the port uses C's own switch implementation (the
   `HAVE_GCC_LABELS=false` branch) and records the deviation.
7. **`__atomic_*` was reported as absent.** There are 24 sites (19 in
   `android.c`), some mixing atomic and non-atomic access to the same object. Now
   specified via `core::sync::atomic` with the same orderings.
8. **Baseline numbers were not reproducible.** `PORTING.md` §0.4 is rewritten with
   the current baseline tool as the single source and the exact commands.
9. **Coverage was `src/`-only** while the binary also links `lib/` (gnulib),
   `lib-src/`, `lwlib/`, `oldXMenu/`, `exec/`, `nt/`. §0.4 and §11 now state the
   real scope: 590 files across the eight C directories.

Tracked but not yet fixed (next turns; they need the spikes or a decision, not
just text):

- `config.h` does not exist in the checkout: it must be produced per target and
  frozen before the `#if` classification can be finalized (§9.2, §11 P-08).
- `dmpstruct.awk` reads C sources to generate `dmpstruct.h`; for Rust sources the
  offset/fingerprint generator must be reimplemented in Rust (§6.4, P-09).
- `sigsetjmp`/`setjmp`/`_setjmp` selection (three C branches) and the Windows
  `__builtin_longjmp` pairing (§5, P-10).
- Custom assertion family: `clobbered_eassert`, `assume` (four platform forms),
  `verify` (two forms), 91 `static_assert`, 184 `eassume` (§3.6, P-11).
- `INLINE` / `EXTERN_INLINE` / `INLINE_HEADER_BEGIN/END` (634 occurrences) and
  function-pointer identity for address-compared functions (§0.1, P-12).
- Attribute macro families (`GCALIGNED_STRUCT` 31, `EXTERNALLY_VISIBLE` 39,
  `ATTRIBUTE_*` 112, `NO_INLINE` 30) plus `cleanup`, `weak_import`,
  `force_align_arg_pointer` and variable alignment (§10.1, P-13).
- `.gdbinit` needs 30+ types/enums besides the six `DEFINE_GDB_SYMBOL` symbols
  (§10.4, P-14).
- The C23 `ckd_add/ckd_sub/ckd_mul` family (287 sites) replaces the non-existent
  `__builtin_add_overflow` mapping (§3.7, P-15).
- `HAVE_STATEMENT_EXPRESSIONS` / `HAVE_TYPEOF` macro forms and the
  multiple-evaluation macros (`AVAIL_ALLOCA`, `SAFE_ALLOCA`) are exceptions to
  the "bind arguments once" rule (§3.1, P-16).

## Outcome

Reviewer B's verdict was "these tools cannot be the mechanical gate". After the
fixes above, the gate now rejects the counterexample they used (stub file fails),
covers every file of the port, and its numbers reconcile with the sources. A
second review of the fixed tools is scheduled after `PORTING.md` v2 lands.
