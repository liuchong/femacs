# emacs/src/gmalloc.c (1908 lines)

Port `src/gmalloc.rs` (**2326 lines**, whole file, C 1-1908); ledger written to P-86's shape
(clause ②: the C line-range table replaces the empty checklist, plus `## Evidence`,
`## Deviations`, `## Owed`).  Reference `emacs/src/gmalloc.c` md5 `fd327d9b580354a68d0be92386c4c4ad`;
the port's md5 is `e999b3f05a0c68ae69ca67aa587cec70` and its sha256
`83d9e2863448884263c2b8ce038a2983c9582203f16e670daa98da3821c82d01` (`md5 -q`/`shasum -a 256`,
final revision).  The 11-line empty
unit it replaced was sha256 `1de8cbd1eba9add0370ae6344b46dee1f08f49cdb466e6e413d96f5d5d599b4b` and its
24-line ledger `e5890b35090224e77ccb502a6bd33f5f46da5650c509a5831ffa1b50f5e40cf8` (both measured
before this lane touched them).

## Decisions

- **The file gate is `all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"))`**, on
  every item *and on every `include!` edge* (P-86 ⑧), which is why the frozen configuration compiles
  nothing from this file.  C's condition is the build's, not an `#if`: `emacs/src/Makefile.in`
  489-490 is `## gmalloc.o if !SYSTEM_MALLOC && !DOUG_LEA_MALLOC, else empty.` +
  `GMALLOC_OBJ=@GMALLOC_OBJ@`, and `configure.ac` 3308-3317 fills `GMALLOC_OBJ` only in the `else`
  arm of `if test "${system_malloc}" = "yes"`, and there only under
  `test "$doug_lea_malloc" != "yes"`.  **The reading that proves membership** is the frozen build's
  own generated makefile plus its generated config: `build/emacs-ref/src/Makefile` 490 is
  `GMALLOC_OBJ=` (empty), `build/emacs-ref/src/config.h` 2342 defines `SYSTEM_MALLOC` (1) and
  `docs/baseline/config/darwin-arm64.macros.tsv` 3615/789/3321 record `SYSTEM_MALLOC` 1,
  `DOUG_LEA_MALLOC` undef, `REL_ALLOC` undef; `docs/baseline/frozen-build-sources.txt` (generated
  from `make -n` in that build directory) therefore lists no `gmalloc.o` for this target.  Both
  feature names are registered (`xtask/Cargo.toml` 597 `no_system_malloc`, 86
  `doug_lea_malloc`); together they are C's `!SYSTEM_MALLOC && !DOUG_LEA_MALLOC`, and one alone
  would compile text C never compiles.
- **The rename chain C 61-71 is carried, and its spelling is the one non-obvious decision.**  C
  `#undef`s the `lisp.h` chain's names and `#define`s `malloc`->`gmalloc`, `realloc`->`grealloc`,
  `calloc`->`gcalloc`, `aligned_alloc`->`galigned_alloc`, `free`->`gfree`,
  `malloc_info`->`gmalloc_info`, so the definitions C 882/1215/1394/1424/1503 emit are the `g*`
  **symbols** while their text names stay `malloc`/`free`/`realloc`/`calloc`/`aligned_alloc`; C
  1657-1661's `#undef`s re-expose the plain names and C 1663-1691's definitions provide the plain
  symbols.  The port keeps C's text name on each of the ten functions and carries the symbol with
  `#[unsafe(export_name)]` (PORTING.md 0.2's spelling for a definition whose text name and symbol
  differ, the gnulib `rpl_*` mechanism) - so C 882's item is `fn malloc` with
  `export_name = "gmalloc"`, C 1663's is `fn rpl_malloc` with `export_name = "malloc"`, and the same
  for the other four.  **One Rust item per symbol** is what forces this: the five plain definitions
  cannot also be *named* C's text name (`E0428`).  Five macros over the `g*` spellings
  (`gmalloc!`/`grealloc!`/`gcalloc!`/`galigned_alloc!`/`gfree!`) let the wrapper bodies keep C 1666
  etc.'s tokens, and one more (`free!`) keeps C 1233's token in `cfree`, whose callee is the `g`
  function because C 1226-1235 sits inside the rename region.  The `malloc_info` name (C 71) lives on
  as a marker macro over the union the port declares as `gmalloc_info` (C's post-rename name).
- **Every `#if` of the file carries both arms**, as `#[cfg]` on the items (and the includes) it
  gates; the gate table below names each one and its frozen value.  `min` (C 1371) and `abort`
  (C 383) are not on this unit's include graph and stand at their use sites (the
  `src/w32term.rs` 1683 shape); `sbrk` stands at C 1463's site for the same reason.
- **`<malloc.h>` (C 28), `<pthread.h>` (C 46) and `<sys/mman.h>` (C 359) have no file in the
  reference tree and no mirror**; each stands in the unit at its own `#include` line (P-88 ①).
  Nothing gmalloc.c names from `<malloc.h>` is used (the three hooks are declared by C 312-314 and
  defined by the port), so that arm carries no item.  The measured surfaces are in `## Evidence`
  (darwin-arm64 clang): `pthread_mutex_t` 64 bytes/align 8 with `PTHREAD_MUTEX_INITIALIZER`'s first
  word `0x32AAABA7`, `PROT_READ` 0x01/`PROT_WRITE` 0x02, `int mprotect(void *, size_t, int)`.
- **This piece is *not* `text-only`**: the arm flip type-checks on this machine (one rustc, no
  second toolchain) and the mutation test shows its reading reaches this unit's own bodies
  (Finding 3), so `## Evidence` ② ③ are measurements.  What they are *not* is a C compile or a link:
  the frozen configuration builds no `gmalloc.o` at all.

## Gate mapping

| C `#if` / build condition | Port gate | Frozen value |
| --- | --- | --- |
| the file itself (`GMALLOC_OBJ`, i.e. `!SYSTEM_MALLOC && !DOUG_LEA_MALLOC`) | `all (feature = "no_system_malloc", not (feature = "doug_lea_malloc"))` (every item and every include edge) | false (`SYSTEM_MALLOC` 1, `DOUG_LEA_MALLOC` undef) |
| C 24 `#ifdef HAVE_MALLOC_H` | `feature = "have_malloc_h"` (include selection; no item, see Decisions) | false (macros.tsv 1453) |
| C 25 `# if GNUC_PREREQ (4, 2, 0)` (a `#pragma` only) | no item; recorded (a compiler directive, `lib/strftime.rs` 269) | true, inside the false arm above |
| C 30 `#ifndef __MALLOC_HOOK_VOLATILE` | the name-keeper `pub const __MALLOC_HOOK_VOLATILE: ()` (Rust spells the qualifier at the access site) | C defines it (nothing else does) |
| C 34 `#if defined HAVE_PTHREAD` | `feature = "have_pthread"` | true (macros.tsv 1502) |
| C 45/199/474/743/1163 `#ifdef USE_PTHREAD` | `feature = "use_pthread"` (C's own alias, defined at C 35; the `alloc.rs` 286 `USE_VALGRIND` shape) | true under C 34 |
| C 49-53 `#if !defined HAVE_MALLOC_H \|\| (__GLIBC__ > 2 \|\| __GLIBC_MINOR__ >= 34)` | `any (not (feature = "have_malloc_h"), feature = "glibc_2_34")` | true by the first half (`__GLIBC__` is undefined: 0 > 2 is false) |
| C 54-57 the same with `>= 24` | `any (not (feature = "have_malloc_h"), feature = "glibc_2_24")` | true by the first half |
| C 90 `#ifdef MSDOS` | `feature = "msdos"` | false |
| C 234/523/1693 `#ifdef GC_MCHECK` | `feature = "gc_mcheck"` | false |
| C 339-341 `#if defined DOUG_LEA_MALLOC \|\| defined SYSTEM_MALLOC` (the `static` on `__malloc_extra_blocks`) | `feature = "doug_lea_malloc"` on the `static` arm (C's `SYSTEM_MALLOC` half is the file gate's own negation, so only the other half is reachable inside the gate) | false |
| C 347 `#if defined GC_MALLOC_CHECK && defined GC_PROTECT_MALLOC_STATE` | `all (feature = "gc_malloc_check", feature = "gc_protect_malloc_state")` | false |
| C 740 `#ifdef GC_MALLOC_CHECK` / C 743 `#elif defined (USE_PTHREAD)` / C 745 `#else` | the three arms under `feature = "gc_malloc_check"`, `all (not (...), feature = "use_pthread")`, `all (not (...), not (feature = "use_pthread"))` | the second (`HAVE_PTHREAD` 1) |
| C 902 `#if !(defined (_LIBC))` | `not (feature = "libc")` | true (`_LIBC` undef) |
| C 1227 `#ifdef weak_alias` | `feature = "weak_alias"` | false (macros.tsv has no `weak_alias`) |
| C 1462 `#if !defined (__GNU_LIBRARY__) \|\| defined (__UCLIBC__)` | `any (not (feature = "__gnu_library__"), feature = "__uclibc__")` | true by the first half |
| C 1477 `#ifdef HAVE_SBRK` | `feature = "have_sbrk"` | true (macros.tsv 1526) |
| C 1640 `#if defined _SC_PAGESIZE \|\| !defined HAVE_GETPAGESIZE` | `any (feature = "_sc_pagesize", not (feature = "have_getpagesize"))` | true (`_SC_PAGESIZE` 29, macros.tsv 4628) |
| C 1693's arm's C 1872 `#ifdef __GNU_LIBRARY__` | `feature = "__gnu_library__"` | false |

## C line ranges

The rows are contiguous and ascending and cover C 1-1908 (blank lines included, so the gate has no
uncovered line).  `docs/port-map.md` 447 lists one target for this reference file, and both
`check-dead-file-coverage.py` and `check-ledger-ticks.py` bind it to this ledger.

| C | block | arm |
| --- | --- | --- |
| 1-21 | the file's opening declaration block | dropped (P-86 ① / PORTING.md 0.1-19) |
| 22 | C's `#include <config.h>` | ungated (every unit includes `config.rs`) |
| 24-29 | C 24's `#ifdef HAVE_MALLOC_H`, the `GNUC_PREREQ` `#pragma` and `# include <malloc.h>` | `have_malloc_h`; no item (Decisions) |
| 30-32 | C 30's `#ifndef __MALLOC_HOOK_VOLATILE` / `#define` / `#endif` | name-keeper |
| 34-36 | C 34's `#if defined HAVE_PTHREAD` / `#define USE_PTHREAD` / `#endif` | `have_pthread` (name-keeper) |
| 38-43 | the six system includes (`stddef`, `stdlib`, `string`, `limits`, `stdint`, `unistd`) | file gate on each edge |
| 45-47 | C 45's `#ifdef USE_PTHREAD` / `#include <pthread.h>` / `#endif` | file gate + `use_pthread`; the header's surface stands in the unit (P-88 ①) |
| 49-57 | C 49-53's `__after_morecore_hook`/`__morecore` and C 54-57's `__malloc_initialize_hook` declarations | both glibc arms recorded; the three names are defined at C 312-314 (a Rust `extern` declaration plus a definition of one name in one crate root is `E0428`) |
| 59 | C's `#include "lisp.h"` | file gate on the edge |
| 61-71 | the five `#undef`s and the six `#define`s | the rename chain (Decisions); `malloc_info` is the marker macro |
| 73-76, 277-281 | C's `extern "C"` / `#undef extern` scaffolding | no counterpart |
| 78-93 | the five allocator prototypes and C 90-93's `#ifdef MSDOS` pair | file gate; the declarations are of functions defined below (dropped as at C 49-57), the MSDOS arm recorded |
| 95-102 | the block/fragment comment, `BLOCKLOG`, `BLOCKSIZE`, `BLOCKIFY` | file gate (three `macro_rules!`) |
| 104-110 | the `HEAP` and `FINAL_FREE_BLOCKS` comments and defines | file gate (`HEAP` a macro, `FINAL_FREE_BLOCKS` a `const` - C's body is a literal) |
| 112-146 | `typedef union … malloc_info` | file gate; declared as `gmalloc_info` with named members for the anonymous aggregates and measured layout assertions |
| 148-162 | the `_heapbase`/`_heapinfo`/`_heapindex`/`_heaplimit` declarations | file gate; defined at C 317/320/323/326 |
| 164-181 | `struct list`, `_fraghead`, `struct alignlist`, `_aligned_blocks` | file gate; the two structs with measured assertions, `_fraghead` zeroed (bss) |
| 183-197 | the instrumentation declarations and the six internal prototypes | file gate; defined at C 329-332 and C 679/870/959/1205/1266/1382 |
| 199-227 | the four locking macros and their empty `#else` arm | file gate + `use_pthread` / `not (use_pthread)` |
| 229-232 | `__malloc_initialized`, `__malloc_initialize` | file gate; defined at C 335/C 551 |
| 234-275 | C 234's `#ifdef GC_MCHECK` block (`enum mcheck_status`, `mcheck`/`mprobe`/`mtrace`/`muntrace`, `struct mstats`, `mstats`) | file gate + `gc_mcheck`; `mcheck`/`mprobe` are defined at C 1882/1902, so only the other three are declared |
| 283-303 | the section's opening block | dropped (P-86 ① / 0.1-19) |
| 304 | C's `#include <errno.h>` | file gate on the edge (C's `errno` is the mirror's accessor) |
| 306-314 | `gmalloc_hook` and the three hook objects | file gate; `__morecore`'s one definition point is C 1485 |
| 316-345 | `_heapbase`, `_heapinfo`, `_heapindex`, `_heaplimit`, the four counters, `__malloc_initialized`, C 337-342's `__malloc_extra_blocks` (with C 339-341's `static` arm) and `heapsize` | file gate (see the gate table for `__malloc_extra_blocks`) |
| 347-391 | C 347's `GC_MALLOC_CHECK && GC_PROTECT_MALLOC_STATE` arm: the comment, C 358-359's two includes, the three state variables, `protect_malloc_state`, and C 387/389-391's two `PROTECT_MALLOC_STATE` definitions | file gate + both sub-gates; the `<sys/mman.h>` surface stands in the unit, `abort` at its use site (Owed) |
| 394-421 | `align` | file gate; C's `PTRDIFF_MAX < size` promotion and `wrapping_neg`-free arithmetic are spelled |
| 423-451 | `get_contiguous_space` | file gate |
| 454-472 | `register_heapinfo` | file gate |
| 474-518 | C 474's `#ifdef USE_PTHREAD` arm: the two mutexes, `_malloc_thread_enabled_p`, the three atfork handlers, `malloc_enable_thread` | file gate + `use_pthread` |
| 520-546 | `malloc_initialize_1` (with C 523-525's `GC_MCHECK` arm) | file gate; the `mcheck (NULL)` arm under `gc_mcheck` |
| 548-560 | `__malloc_initialize` | file gate |
| 562-563 | `morecore_recursing` | file gate |
| 564-675 | `morecore_nolock` (with C 623's `goto got_heap`) | file gate; the goto is the labeled block, C's label name kept |
| 677-868 | `_malloc_internal_nolock` (C 693-696's `#if 0`, C 740-747's three arms, C 751/C 817's `goto out`) | file gate; the `#if 0` body dropped (the `exec/trace.rs` 1605 precedent), the three arms as gated blocks, the goto the labeled block |
| 870-880 | `_malloc_internal` | file gate |
| 882-900 | `malloc` (the `gmalloc` symbol) | file gate |
| 902-929 | C 902's `#if !(defined (_LIBC))` arm: `_malloc`, `_free`, `_realloc` | file gate + `not (libc)`; C 907-909's declarations are of the definitions below |
| 930-949 | the section's opening block | dropped |
| 951-955 | `gfree_hook`, `_aligned_blocks` | file gate |
| 957-1201 | `_free_internal_nolock` (C 991's `switch`, C 1163-1167's two arms) | file gate; `switch` -> `match`, both arms gated |
| 1203-1211 | `_free_internal` | file gate |
| 1213-1224 | `free` (the `gfree` symbol) | file gate |
| 1226-1235 | C 1226's `#ifdef weak_alias` / `#else` / `cfree` / `#endif` | file gate + `weak_alias` / `not (weak_alias)` (the `lib/euidaccess.rs` 197-204 shape) |
| 1236-1255 | the section's opening block | dropped |
| 1257-1258 | `grealloc_hook` | file gate |
| 1260-1380 | `_realloc_internal_nolock` (C 1286's `switch`, C 1297/1350/1370's `goto out`, C 1371's `min`) | file gate; `match` + labeled block, `min` declared at its use site (Owed) |
| 1382-1392 | `_realloc_internal` | file gate |
| 1394-1404 | `realloc` (the `grealloc` symbol) | file gate |
| 1405-1422 | the section's opening block | dropped |
| 1424-1442 | `calloc` (the `gcalloc` symbol) | file gate |
| 1443-1458 | the section's opening block | dropped |
| 1460-1469 | C 1460's comment, C 1462's `#if !defined (__GNU_LIBRARY__) || defined (__UCLIBC__)` / `#define __sbrk sbrk` / `#else` / the `__sbrk` declaration / `#endif` | file gate; `sbrk`'s own declaration stands at C 1463's site (P-88 ①, Owed) |
| 1471-1483 | `gdefault_morecore` (C 1477's `#ifdef HAVE_SBRK`) | file gate; the arm under `have_sbrk` |
| 1484-1485 | C's `__morecore` definition-with-initializer | file gate; the object's one definition point |
| 1487-1501 | the section's opening block | dropped |
| 1503-1584 | `aligned_alloc` (the `galigned_alloc` symbol) | file gate; C 1511's `- size` is `wrapping_neg` |
| 1586-1594 | the `memalign` comment and `memalign` | file gate |
| 1596-1613 | `posix_memalign` | file gate; C's `sizeof (void *)` is `size_of::<*mut c_void> ()` |
| 1615-1633 | the section's opening block | dropped |
| 1635-1638 | C 1635's `#ifndef HAVE_MALLOC_H` / the `valloc` declaration / `#endif` | file gate; the declaration is of the function C 1648 defines (dropped) |
| 1640-1644 | C 1640's `#if defined _SC_PAGESIZE || !defined HAVE_GETPAGESIZE` / `# include "getpagesize.h"` / `#elif` / the `getpagesize` declaration / `#endif` | file gate + its condition; the `#elif` arm's declaration is `include/unistd.in.rs`'s (recorded rather than repeated) |
| 1646 | `pagesize` | file gate |
| 1648-1655 | `valloc` | file gate |
| 1657-1661 | the five `#undef`s | the rename chain's boundary (Decisions) |
| 1663-1691 | `malloc`, `calloc`, `free`, `aligned_alloc`, `realloc` (the plain symbols) | file gate; the `rpl_*` items with C's names in `export_name` |
| 1693-1714 | C 1693's `#ifdef GC_MCHECK` and the section's opening block | file gate + `gc_mcheck`; the block dropped |
| 1716 | C's `#include <stdio.h>` | file gate + `gc_mcheck` on the edge |
| 1718-1731 | the old hook values, `abortfunc`, the five magic-number macros | file gate + `gc_mcheck`; all five as `macro_rules!` |
| 1733-1737 | `struct hdr` | file gate + `gc_mcheck`, with the measured assertion |
| 1739-1761 | `checkhdr` (C 1743's `switch` with two macro `case` labels) | the switch is a `match` with guards (a macro value is not a Rust pattern) |
| 1763-1797 | `freehook` | file gate + `gc_mcheck` (the `free`/`memset` calls are the plain symbols C 1657 re-exposed) |
| 1799-1814 | `mallochook` | ditto |
| 1816-1848 | `reallochook` | ditto |
| 1850-1878 | `mabort` (C 1872-1877's `#ifdef __GNU_LIBRARY__` arms) | file gate + `gc_mcheck`; `__libc_fatal` declared at its use site (P-88 ①) |
| 1880-1881 | `mcheck_used` | file gate + `gc_mcheck` |
| 1882-1900 | `mcheck` | file gate + `gc_mcheck` |
| 1902-1906 | `mprobe` | file gate + `gc_mcheck` |
| 1907-1908 | C 1908's `#endif /* GC_MCHECK */` | file gate + `gc_mcheck` |

## Findings

1. **The arm flip is *not* vacuous on this unit: every one of its errors is anchored in an
   include-graph copy, and a mutation test says the unit's own bodies are reached** (Evidence ②③).
   The four `E0428`s are the one class this batch *creates* and the mirror round owns (Owed 5): C
   includes `<stdlib.h>` (C 39) and this file *defines* `malloc`/`realloc`/`calloc`/`free`, so
   `lib/include/stdlib.rs`'s four declarations sit in the same crate root as the definitions.  C
   resolves the same collision with C 61-66's `#undef`/`#define`; Rust's remedy is PORTING.md P-39's
   glob-imported `pub mod` + `use`, which is that mirror's job.  The fifth error (`E0658`,
   `c_variadic` at `src/include/lisp.rs` 3761) appeared while this lane was working - the
   mirrors are shared and moving - and is that mirror's, not this unit's.
2. **A multi-line `#[cfg]` above an `include!` edge loses its gate in `xtask`'s `materialize`**
   (measured, and the reason the first frozen `build-tu` of this unit was wrong): `materialize`
   recognises the gate from the *line* before the `include!`, so only a one-line attribute counts.
   This lane's first unit text had C 59's `<lisp.h>` edge ungated and two gated edges written over
   two physical lines; the frozen object came out **1240 bytes with `xfree_wrapper`,
   `flush_stack_barrier` and `cons_to_integer` in it** (i.e. `include/lisp.rs`'s chain had been
   compiled into a build that must compile nothing).  Every gated edge is now one line
   (`src/gmalloc.rs` 62/83-93/137/458/553/1985/2057), and the frozen object is the empty
   one (Evidence ①).  The tool side of this is Owed 7.
3. **`extract-abi.py` cannot see C's `#define` rename chain, so the five `rpl_*` items read as
   invented symbols** (Evidence ⑤).  C 66-70's renames make C 882/1215/1394/1424/1503's definitions
   the `g*` symbols; `whole_file_exported_names` scans the *text* for definitions, where those
   names appear only as macro bodies, so `g*` can never answer for a `#[unsafe(export_name)]`.  The
   alternative spellings were measured and cost more: naming those items `gmalloc`/… directly makes
   each body compare against C's *other* same-named definition (the C 1657-1691 wrappers), which is
   5 missing-token rows plus seven family-floor rows that no notes row can clear
   (`alias_count_problems` does not consult `macro-body-notes.tsv`).  The port's spelling clears all
   of those; the five invented-symbol rows are Owed 2.
4. **The `static` arm of C 339-341 cannot be compiled together with the file gate**, in C or in the
   port: `defined SYSTEM_MALLOC` is the gate's own negation.  C 337-342's `static` therefore hangs
   on the reachable half (`DOUG_LEA_MALLOC`) and the ledger records it (measured: the `doug_lea_malloc`-only
   pre-flight arm compiles that one item, 776 bytes).
5. **The frozen `build-tu` of the second revision failed** (`E0425: cannot find type size_t`, on
   the `static __malloc_extra_blocks` arm, which was then gated by
   `not (feature = "no_system_malloc")` - true in a frozen build).  That is Finding 4's cause and is
   recorded here rather than hidden: it is the one compile failure this lane produced, and the fixed
   gate is in the unit.

## Evidence

- **frozen `build-tu`** (`cd femacs && CARGO_TARGET_DIR=/tmp/gm-lane cargo run -q -p xtask --
  build-tu src/gmalloc.c`) -> `built src/gmalloc.rs -> ../build/femacs/gmalloc.o`, **exit 0,
  336 bytes**, 0 warnings.  This is the sixth run of this command (budget 3, +3): the first
  produced Finding 2's 1240-byte object, the second failed with Finding 5's `E0425`, and the last
  four are the spellings this lane settled on (the rename chain, `cfree`, the six macros' gates);
  each of the last four printed 336 bytes.  That size is the *empty object*: every item of this unit is behind the
  file gate, so the frozen compile sees only `include/config.rs`'s chain (the `w32heap`/`ralloc`
  reading).  It is a membership reading - "this configuration compiles no `gmalloc.o`" - not a
  build, and 336 bytes says nothing about any item's content.
- **arm flip, arm 1** (`rustc --edition 2024 --crate-type=lib --emit=obj --crate-name gmalloc
  -C panic=abort -C codegen-units=1 -C opt-level=0 --cfg 'feature="no_system_malloc"'
  --cfg 'feature="have_pthread"' --cfg 'feature="use_pthread"' --cfg 'feature="have_sbrk"'
  --cfg 'feature="_sc_pagesize"' build/femacs/units/gmalloc/_root.rs`) -> **exit 1, 5 errors, 0 of
  them anchored in the unit**: `lib/include/stdlib.rs` 18/20/22/24, 4 x `E0428`
  (`malloc`, `realloc`, `calloc`, `free` - the notes point back at `_root.rs` 1969/1978/1987/2005,
  this unit's definitions), and `src/include/lisp.rs` 3761, 1 x `E0658` (`verror`'s
  `core::ffi::VaList`; that mirror was edited by another lane during this batch - Findings 1).
  4 warnings, all `unused doc comment` on `/// C:` lines (the house warning class).
- **arm flip, arm 2** (the same root, `--cfg 'feature="no_system_malloc"'` plus `gc_mcheck`,
  `gc_malloc_check`, `gc_protect_malloc_state`, `libc`, `weak_alias`, `have_malloc_h`,
  `glibc_2_34`, `glibc_2_24`, `__gnu_library__`, `__uclibc__`, `msdos`) -> **exit 1, the same 5
  errors, 0 in the unit**.  Between them the two arms carry both sides of every sub-gate the file
  has; `xtask probe`'s own runs on the preceding revisions gave the same shape (4 errors, all
  `lib/include/stdlib.rs`) before that mirror's `E0658` existed.  `xtask probe` was run four times
  (budget 3, +1): two arm-1 runs, one arm-2 run and the mutation run, each on the revision current
  at the time; the two readings above are `rustc` on the `materialize`d `_root.rs` the last
  `build-tu` wrote, which is the same expansion `probe` performs.
- **mutation test, both readings** (one injected type error, `let _mutation_probe: c_int = result;`
  in `align`'s declarations, removed again): (a) the flip's materialized root, mutated ->
  **exactly one new error**, `E0308` at the injected line (`L1_mut.rs:646:32`; the clean listing
  has no unit-anchored error); (b) the arm-reach pre-flight built from the unit's own text ->
  **exactly one error**, `E0308` at the injected line.  The unit was then restored byte-for-byte:
  sha256 `83d9e2863448884263c2b8ce038a2983c9582203f16e670daa98da3821c82d01`, md5
  `e999b3f05a0c68ae69ca67aa587cec70`.  So both readings bear on this unit's text.
- **arm-reach pre-flight** (P-86 ⑦'s fallback; the same unit text with its `include!` edges dropped
  and a stub prelude carrying exactly the names the graph supplies - `size_t`, `ptrdiff_t`,
  `uintptr_t`, `PTRDIFF_MAX`, `SIZE_MAX`, `INT_WIDTH`, `errno ()`/`__error`, `ENOMEM`, `EINVAL`,
  `stderr`, `memset`/`memcpy`/`memmove`/`fprintf`, `emacs_abort`, `getpagesize`; 3 + 60 lines) ->
  five configurations, all **exit 0**: a (frozen sub-gate values: `no_system_malloc`,
  `have_pthread`, `use_pthread`, `have_sbrk`, `_sc_pagesize`) **142232 bytes**; b (the other side of
  every sub-gate) **155928**; c (as a, `use_pthread` off - the empty `LOCK`/`UNLOCK` arms)
  **137192**; d (as a, `have_getpagesize` on) **142232**; e (`doug_lea_malloc` only - C 337-342's
  `static` arm) **776**.  Warnings: `unused doc comment` only.  It is *not* the flip: the stubs are
  this lane's, so a green pre-flight says the unit's text is well-typed against the names its graph
  supplies, not that those names are the SDK's.
- **layout measurements** (clang, darwin-arm64; `printf` of `sizeof`/`_Alignof`/`offsetof` on a
  probe that copies C 113-146, C 164-181, C 1733-1737, C 262-271 and C 238-246 verbatim, plus
  `<pthread.h>` and `<sys/mman.h>`): `gmalloc_info` **24/8** (`busy.type` 0, `busy.info` 8,
  `frag.nfree` 8, `frag.first` 16, `info.size` 8, `free.size` 0, `free.next` 8, `free.prev` 16);
  `struct list` **16/8** (prev 8); `struct alignlist` **24/8** (aligned 8, exact 16); `struct hdr`
  **16/8** (magic 8); `struct mstats` **40/8**; `enum mcheck_status` **4/4** (-1, 0, 1, 2, 3);
  `pthread_mutex_t` **64/8** with a static `PTHREAD_MUTEX_INITIALIZER`'s eight words
  `32aaaba7 0 0 0 0 0 0 0`; `PROT_READ` 0x01, `PROT_WRITE` 0x02; `SIZE_MAX` 0xffffffffffffffff,
  `PTRDIFF_MAX` 0x7fffffffffffffff.  Nineteen `const _: () = assert!(…)` lines in the unit pin the first
  ten of those; the `pthread_mutex_t`/`PROT_*` readings are in the stand-in sections.
- **`python3 tools/extract-abi.py --check emacs/src/gmalloc.c src/gmalloc.rs`** -> 6 rows and
  `FAIL`: five `fn rpl_malloc|rpl_calloc|rpl_free|rpl_aligned_alloc|rpl_realloc carries
  #[unsafe(no_mangle)] but the reference does not define a non-static function of that name
  (invented exported symbol)` (Finding 3, Owed 2) and
  `line count drifted 21.9%: C 1908 vs Rust 2326` (Owed 1).  Every function, macro, type, field,
  file-scope object, `switch`/`goto` token and every family floor is accounted for: the nine
  remaining rows this lane measured were cleared by the `docs/ports/macro-body-notes.tsv` rows it
  appended (`malloc`/`realloc`/`calloc`/`aligned_alloc` -> the `g*` symbol names,
  `morecore_nolock`/`_malloc_internal_nolock`/`_realloc_internal_nolock` -> `goto`,
  `_free_internal_nolock`/`_realloc_internal_nolock` -> `switch,case,default`).
- **the whole-tree gates** (last lines; findings on this lane's files): see the closeout readings in
  the final report; `check-dead-file-coverage.py` reports `117 ledger(s) bound (a C range table),
  0 failure(s)` with this ledger's 80 rows covering `src/gmalloc.c` 1908 lines
  (`info docs/ports/gmalloc.md: 80 range row(s) cover src/gmalloc.c (1908 lines; 70 blank line(s)
  between blocks need no row)`).

## Deviations

1. **The ten definitions carry C's symbol in `#[unsafe(export_name)]`** and the five plain ones an
   `rpl_*` item name (Decisions).  Symbols are C's exactly (`gmalloc`, `grealloc`, `gcalloc`,
   `galigned_alloc`, `gfree`, `malloc`, `calloc`, `free`, `aligned_alloc`, `realloc`).
2. **Five `macro_rules!` over the `g*` spellings plus one `free`** exist so the wrapper bodies and
   `cfree` keep C's tokens (C 1666/1672/1678/1684/1690 and C 1233).
3. **The `malloc_info` union is declared as `gmalloc_info`** (C's post-rename name, C 71) with
   named members for C's three anonymous aggregates (`gmalloc_info_busy`,
   `gmalloc_info_busy_info`, `gmalloc_info_busy_info_frag`, `gmalloc_info_free`); the `type` field
   keeps C's name as `r#type`.
4. **Forward declarations are recorded, not transcribed** (C 49-57, C 78-93, C 189-197, C 200-201,
   C 229-232, C 907-909, C 1635-1638): each declares a function or object this same unit defines,
   and a Rust `extern` declaration plus a same-crate definition of one name is `E0428`.
5. **The `#elif !defined getpagesize` arm (C 1642-1644) is recorded rather than transcribed**: the
   declaration it holds is already in `include/unistd.in.rs`, which this unit includes.
6. **`#if 0` (C 693-696) is dropped and recorded** (the `exec/trace.rs` 1605 precedent); the
   C 25 `#pragma` likewise has no Rust item.
7. **C's implicit conversions are spelled**: `PTRDIFF_MAX as size_t` (C 405), `size.wrapping_neg ()`
   for C 1511/1478/635/646/1118's unsigned negations, `as ptrdiff_t`/`as size_t`/`as c_int` at the
   assignments C 470/471/771/772/1360-1361 etc. leave implicit, and `sizeof x` as
   `core::mem::size_of::<T> ()`.
8. **C's chained assignments are split and their index reads are pinned before the stores**
   (C 536, C 787, C 843-845, C 853-854): the C text's evaluation order for the lvalue subexpressions
   is unspecified, and the port reads both indices from the pre-store state.
9. **C's `switch`es are `match`es** (C 991, C 1286, C 1743, C 1854): the two `checkhdr` `case`
   labels are *macro* values and cannot be Rust patterns, so that match is written with guards
   (`_ if (*hdr).magic == MAGICFREE! ()`) - the `default:` arm stays the catch-all.
10. **Locals C leaves uninitialised are zero-initialised** (PORTING.md 1.5): `result`, `block`,
    `blocks`, `lastblocks`, `start`, `i`, `next`, `prev`, `curbrk`, `newinfo`, `oldinfo`,
    `morecore_recursing`, `save`, `status`, `hdr`, `msg`, `l`, `osize`, `mem`, `adj`, `lastadj`,
    `pagesize` and the `hook` locals; `status` takes `MCHECK_OK`, which is the enum's zero.
11. **`sbrk` (C 1463), `min` (C 1371), `abort` (C 383) and `abortfunc`'s `__libc_fatal` (C 1873)
    stand at their use sites** because no mirror on this unit's include graph carries them (P-88 ①;
    Owed 6).
12. **`errno` goes through the mirror's accessor**: C's `errno = save;` is `*errno () = save;`
    (`include/errno.rs` 14-17: `#define errno (*__error ())` on this target).
13. **`<malloc.h>`, `<pthread.h>` and `<sys/mman.h>` surfaces stand in the unit** (P-88 ①), each
    with the header's own line in a `/// C:` line and the measured readings in `## Evidence`.
14. **The line count needs a tolerance row** (Owed 1): 2326 vs 1908 is 21.9%, because every item
    carries its gate (158 `#[cfg]` lines), every declaration carries its `/// C:` line and every
    block a `// C <line>` citation (226 of them), and the three stand-in sections are the port's own
    text.

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `gmalloc`, tolerance >= 0.219,
   basis `total`, with Deviations 1/2/4/7/10/13/14's reasons.  Not in this lane's write set; until it
   lands `extract-abi.py --check` stays `FAIL` with that row.
2. **The five invented-symbol rows** (Evidence ⑤): `lib/include/stdlib.rs`'s declarations are
   the mirror round's (Owed 5) and the `g*` names need either a `#define`-aware pass in
   `tools/extract-abi.py`'s `whole_file_exported_names` or a `PORT_ONLY_EXPORTS`-style registration
   (`src/extract-abi.py` 1623) - owner: the extractor/integrator.  Until then the port cannot be
   "clean" for this file by any spelling that keeps C's ten symbols.
3. **Feature registration in `xtask/Cargo.toml`**: `have_malloc_h`, `use_pthread`,
   `glibc_2_34`, `glibc_2_24`, `gc_mcheck`, `gc_protect_malloc_state`, `have_sbrk`, `_sc_pagesize`,
   `have_getpagesize`, `__gnu_library__`, `__uclibc__` (11 names; `no_system_malloc`,
   `doug_lea_malloc`, `have_pthread`, `gc_malloc_check`, `weak_alias`, `libc`, `msdos` are already
   registered).  The `probe` readings above pass them with `--cfg`, so the readings do not depend on
   the registration, but `feature-coverage.py`/`dead-arms` cannot see them without it.
4. **Layout registration**: `gmalloc_info` (+ its three nested aggregates), `list`, `alignlist`,
   `hdr`, `mstats` and `mcheck_status` belong in `tools/gen-layout-fields.py`'s `STRUCTS` so
   `xtask layout-asserts` owns the ten assertions this unit now writes by hand, with the measurements
   of `## Evidence` as the baseline.
5. **The mirror round's reconciliation of `lib/include/stdlib.rs`**: its four declarations
   (`malloc`, `realloc`, `calloc`, `free`) collide with this unit's definitions (4 x `E0428` in both
   arms) and need PORTING.md P-39's glob-imported `pub mod` + `use` shape, exactly as
   `src/include/w32heap.rs` and `nt/inc/sys/include/resource.rs` do for `src/w32heap.rs`.  The
   `E0658` at `src/include/lisp.rs` 3761 (`verror`'s `VaList`) is the same mirror round's.
6. **A mirror home for `sbrk`, `abort`, `min`**: `lib/include/unistd.in.rs` (or the frozen
   `<unistd.h>` mirror) for the first, `lib/include/stdlib.rs` for the second, and lisp.h's
   `min`/`max` mirror for the third; until then the unit declares them at their use sites (P-88 ①).
7. **`xtask`'s `materialize` and multi-line gates** (Finding 2): a `#[cfg]` above an `include!` is
   read from one physical line only, which silently compiles a gated edge in the frozen
   configuration (measured: 1240 bytes with the `lisp.rs` chain in this unit's first draft).  Either
   `materialize` should parse the attribute or the rule should be a check - owner: the xtask/tooling
   round.
8. **`docs/ports/macro-body-notes.tsv`**: ten rows appended by this lane in all - nine (the five
   rename names and the four `switch`/`goto` functions) and then one more for `aligned_alloc`.
   **Measured hazard (twice in twenty minutes)**: the shared file is being rewritten whole by
   concurrent lanes, and this lane's rows were dropped both times - the first append (eight rows,
   `grep -c` 8) was gone by 17:01, and after the nine-row re-append at 17:03 the `aligned_alloc`
   row was gone again at 17:04 (`extract-abi.py --check` went 6 rows -> 7 rows and back to 6 when it
   was re-appended).  The reading in `## Evidence` is the state at 17:06 (file 1589 lines); a
   rewrite after that can drop rows again.  Owner: the integrator - append in place (or one file per
   reference stem), and verify this file's rows survive before the batch closes.
9. **A real link and a real configuration** would be the readings that could move this file out of
   "carried text": the frozen configuration has no `gmalloc.o` and this machine has no
   `rel_alloc`/`!SYSTEM_MALLOC` build (the `ralloc.md` Owed notes are the same class).

## Not claimed

- **The frozen configuration compiles nothing here** and the 336-byte object is the empty object;
  the port is not "built" by any reading of this tree, and `docs/baseline/frozen-build-sources.txt`
  lists no `gmalloc.o`.
- **The arm flip is a type-check under this machine's `rustc`** (and, in the last two readings,
  `rustc` on the same materialization rather than the `xtask probe` wrapper), not a C compile and
  not a link: 5 errors, all in include-graph copies.
- **No link, no run, no runtime behaviour**: the allocator's policy, the fragment lists, the
  `goto`-shaped control flow, the state protection and the mcheck hooks are transcribed, not
  exercised.  `GNU_MALLOC`/`DOUG_LEA_MALLOC` configurations do not exist on this machine.
- **Every naming decision in Decisions/Deviations 1-3 is a port spelling, not C's text**: the
  `rpl_*` names and the five `g*` macros are this lane's, and a reviewer should read them against
  C 61-71/1657-1691 rather than trusting the checker's silent rows for the wrapper bodies (which
  the checker never compares - the C name has two definitions and `c_body_text` takes the first).
- **The pthread/`<malloc.h>`/`<sys/mman.h>` readings are this machine's** (darwin-arm64,
  one rustc/clang): `PTHREAD_MUTEX_INITIALIZER`'s other words, `pthread_mutex_t` on another libc,
  and the `mprotect` prototype are the documented ones, not quotations from a second SDK.
- **Nothing about the reference's behaviour is claimed from this ledger**: the C line-range table,
  the gate table, the check readings and the readings above are the whole evidence, and
  `extract-abi.py --check` still exits `FAIL` with the six rows of Evidence ⑤.
