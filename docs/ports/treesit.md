# emacs/src/treesit.c (5634 lines)

> **P-86 unit.**  `HAVE_TREE_SITTER` is `undef` in the frozen configuration, so this file's body
> sits entirely under a condition the frozen configuration leaves false: the port transcribes it
> (**both arms of every `#if`**), and its coverage evidence is the C line-ranges table below
> plus `## Evidence`, not the checklist (whose live-line filter leaves two entries, both from the
> text *outside* the gate).  This round lands the first slice, C 29-946; the rows the `arm` column
> marks *not transcribed this round* are owed, and both the table and `## Evidence` say so.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] treesit-available-p  (Ftreesit_available_p, 0/0, doc de003bcfb47b, line 5295)

## Functions defined — 1
- [x] syms_of_treesit  (line 5312)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file gate (`#if HAVE_TREE_SITTER`, C 29) | `feature = "have_tree_sitter"` (on every item *and* on C 24-27's include graph) | a tree-sitter configuration |
| C 34 `#ifdef WINDOWSNT` … C 315 (the DLL loader) | `all (feature = "have_tree_sitter", feature = "windowsnt")` | a Windows configuration with tree-sitter |
| C 38-42, 101-105, 186-190, 252-256 (`TREE_SITTER_LANGUAGE_VERSION >= 15`) | `feature = "treesit_language_version_ge_15"` — **a name this tree has not registered** (see `## Deviations`) | tree-sitter 0.25 or newer |
| C 88-92, 165-169, 237-241, 303-307 (`HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING`) | `feature = "have_ts_tree_cursor_goto_previous_sibling"` — **also unregistered** | a tree-sitter with `ts_tree_cursor_goto_previous_sibling` |

`have_tree_sitter` is the name `xtask/Cargo.toml` 161 registers for C's `HAVE_TREE_SITTER`
(one of the 24 sites `docs/baseline/feature-coverage.tsv` 26 lists, `enabled by:` empty today);
`windowsnt` is the name `src/decompress.rs` and `src/vm-limit.rs` 95 already use for that
configuration.  The two version-derived names have no row anywhere in the tree yet: this file is
their first consumer, and the task's rule is to *name* a missing registration rather than add one to
a shared manifest.  The other two conditions this file names, `HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING`
and `TREE_SITTER_LANGUAGE_VERSION`, are `undef`/absent in the frozen table
(`docs/baseline/config/darwin-arm64.macros.tsv` 1659 and no row), so on this target the frozen side
of each pair is the one that is *not* compiled - and neither is, because the file gate is off too.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's opening declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 21-28 | C 22-27's five includes | `config.h`/`lisp.h` ungated; `buffer.h`/`coding.h`/`treesit.h` under `have_tree_sitter` (C 27's mirror edge is dropped: see `## Deviations`) |
| 29-33 | `#if HAVE_TREE_SITTER` and C 32's *Dynamic loading of libtree-sitter.* banner | file gate; **transcribed** |
| 34-35 | `#ifdef WINDOWSNT` and `# include "w32common.h"` | `windowsnt`; **transcribed** |
| 36-99 | C 37's note and the `#undef` run (with C 38-42's and C 88-92's pairs) | `windowsnt`; no counterpart, recorded at its C position (`## Deviations`) |
| 100-176 | the `DEF_DLL_FN` declarations (with C 101-105's and C 165-169's pairs) | `windowsnt`; **transcribed** (macros written out: `## Deviations`) |
| 177-250 | `init_treesit_functions` | `windowsnt`; **transcribed** |
| 251-313 | the `#define` rename run (with C 252-256's and C 303-307's pairs) | `windowsnt`; no counterpart, recorded; call sites spelled per arm |
| 314-315 | `#endif /* WINDOWSNT */` | the arm's end |
| 316-486 | the file's own commentary | file gate; **transcribed** as text |
| 487-494 | the Constants banner, `TREESIT_BOB_LINECOL`, `TREESIT_EMPTY_LINECOL`, `TREESIT_TS_POINT_1_0` | file gate; **transcribed** |
| 495-533 | the Initialization banner, the nineteen `Vtreesit_str_*` objects, `TREESIT_RECURSION_LIMIT`, `treesit_initialized` | file gate; **transcribed** |
| 534-574 | `load_tree_sitter_if_necessary`, `treesit_initialize` | file gate; **transcribed** |
| 575-621 | the Debugging banner and `treesit_debug_print_parser_list` | file gate; **transcribed** |
| 622-656 | the Loading-language banner, `struct treesit_loaded_lang`, `struct treesit_lang_cache_entry`, the three cache objects | file gate; **transcribed** |
| 657-756 | `treesit_lang_cache_get`, `treesit_lang_cache_put`, `treesit_symbol_to_c_name`, `resolve_language_symbol`, `treesit_find_override_name` | file gate; **transcribed** |
| 757-805 | `treesit_load_language_push_for_each_suffix`, `treesit_language_abi_version` | file gate; **transcribed** |
| 806-946 | `treesit_load_language` | file gate; **transcribed** (this slice's last block) |
| 947-1044 | the four DEFUNs `treesit-language-available-p` … `treesit-grammar-location` | file gate; **not transcribed this round** |
| 1045-1280 | the Linecol section: `TREESIT_DEBUG_LINECOL`, `treesit_debug_print_linecol`, `treesit_buf_tracks_linecol_p`, `restore_restriction_and_selective_display`, `treesit_debug_validate_linecol`, `linecol_offset`, `treesit_make_ts_point` and the two linecol DEFUNs | file gate; **not transcribed** |
| 1281-2057 | the Parsing section (`treesit_check_parser` … `treesit_ensure_parsed`) | file gate; **not transcribed** |
| 2058-2298 | the parser/node object section (`make_treesit_node`, `make_treesit_query`, the deleters, the query-cursor and compiled-query helpers) | file gate; **not transcribed** |
| 2299-2642 | the DEFUNs `treesit-parser-p` … `treesit-parser-set-embed-level` and the Parser API banner | file gate; **not transcribed** |
| 2643-2993 | the rest of the Parser API section (`treesit-parser-root-node` … `treesit-parser-changed-regions`) and the Node API banner | file gate; **not transcribed** |
| 2994-3487 | the Node API section (`treesit-node-type` … `treesit-node-eq`) | file gate; **not transcribed** |
| 3488-4283 | the Query section (`treesit-pattern-expand` … `treesit-query-capture` and the predicate helpers) | file gate; **not transcribed** |
| 4284-5292 | the Navigation section (the cursor helpers and `treesit-search-subtree` … `treesit-subtree-stat`) | file gate; **not transcribed** |
| 5293-5294 | C 5293's `#endif` | the file gate's end |
| 5295-5313 | `Ftreesit_available_p` and C 5306-5313's banner and `syms_of_treesit` head | ungated; live; transcribed in an earlier round |
| 5314-5621 | `syms_of_treesit`'s `#if HAVE_TREE_SITTER` region | file gate; **not transcribed** (recorded by the round that landed `Ftreesit_available_p`) |
| 5622-5634 | C 5623-5631's `#ifdef WINDOWSNT` block and the file's closing braces | `windowsnt` / file end; **not transcribed** |

## Decisions

`emacs/src/treesit.c` @ `fd7afced403`; port `src/treesit.rs` (1216 lines, of which the
transcribed slice is the whole increase from the 48-line live-text-only revision); the gates are
`build-tu src/treesit.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/treesit.c src/treesit.rs` -> **`FAIL`** on
the line-count ratchet only (see `## Evidence` ③, which names the row that has to change).

- **Which text is live.**  `HAVE_TREE_SITTER` is `undef` (macros.tsv 1656), so 53 of 5,634 lines
  are this target's text: C 22-27 (`config.h`, `lisp.h`, `buffer.h`, `coding.h`, `treesit.h`),
  `Ftreesit_available_p` with its `#if HAVE_TREE_SITTER` arm dead (C 5303's `return Qnil;`) and
  `syms_of_treesit`'s one `defsubr`.  That is what the port compiled before this round and what it
  still compiles (the frozen object's symbols are unchanged: `## Evidence` ①).
- **What this round adds.**  The file gate's opening, the Windows loader arm, the file's
  commentary, the constants, the language cache and `treesit_load_language` - C 29-946, the first
  ~1,000-line slice, carried as text under the gates in `## Gate mapping`.  Every added item is
  gated, so the frozen reading is untouched; the arm that would compile them cannot be built on
  this host (`## Evidence` ④⑤).
- **What this round does not do.**  Everything below C 947 stays recorded (the table's
  `not transcribed` rows), and so do the three declarations the transcribed text needs but the port
  does not own: `struct Lisp_TS_Parser` with `XTS_PARSER` (inside `src/treesit.h`'s own recorded
  `#ifdef HAVE_TREE_SITTER` region, `docs/ports/include-treesit.md`), the `TS*` types of
  `<tree_sitter/api.h>`, and that header's `TREE_SITTER_LANGUAGE_VERSION` /
  `TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION`.  This unit names them and does not invent them
  (P-88's shape for a third-party header wants a clang-measured layout, and there is no header to
  measure: `## Evidence` ④), so the `have_tree_sitter` arm's own text names 26 names the port does
  not yet declare - the flip reading in `## Evidence` ⑤ is exactly that list and nothing else.
- **Line-count basis (decided 2026-10-06, status 3.678).**  That table's newest `treesit` row is
  `treesit 0.79 total`, whose number is the measured shortfall against the file's total and whose
  reason names the owed ranges; it supersedes the `live` row this note described, which could only
  ever read as drift.  The note as it stood: the `treesit` row in
  `docs/ports/line-drift-tolerance.tsv` is a `live` row
  (53 live lines vs the port's 1,216), which is the wrong baseline for a P-86 transcript and is why
  `--check` now reports a 2,194% drift: see `## Deviations` (the row is the integrator's, P-86 ⑩).

## Deviations

- **Two unregistered feature names.**  `treesit_language_version_ge_15` and
  `have_ts_tree_cursor_goto_previous_sibling` stand for C's `TREE_SITTER_LANGUAGE_VERSION >= 15` and
  `HAVE_TS_TREE_CURSOR_GOTO_PREVIOUS_SIBLING` pairs.  The tree has no tree-sitter version gate and
  this unit does not add names to `xtask/Cargo.toml` (`## Gate mapping`; the androidvfs
  `android_api_ge_9` precedent).  Both are off in every configuration the tree can build today,
  which is the frozen side of each pair as well.
- **C 27's `#include "treesit.h"` is not an `include!` edge.**  The mirror declares
  `syms_of_treesit`, and this unit defines it, so both in one crate is `E0428` - measured, not
  reasoned: the first flip reading had exactly that error until the edge came out.  No name this
  slice's text needs comes from the mirror anyway (its records are in its own recorded region).
- **The `#undef` run (C 37-99) and the `#define` run (C 251-313) have no Rust counterpart.**  Rust
  has no macro table to remove a name from, and an object-like `#define` renames an identifier, so
  the two runs are recorded at their C positions and the call sites are spelled the way C's
  preprocessor leaves each name: `fn_ts_<func> (...)` under `windowsnt`, `ts_<func> (...)`
  otherwise (the rule `src/decompress.rs` recorded for the same construct, P-22's first consumer).
  **Cost, for the integrator:** this file renames ~60 entry points, so the slice-2+ call sites each
  cost a two-arm `#[cfg]` pair; a wrapper function per name is the cheaper alternative spelling and
  is a PORTING.md §11 decision, not this lane's.
- **`DEF_DLL_FN`/`LOAD_DLL_FN` are written out in long form** in this unit, as
  `src/include/w32common.rs` 171-184 defers them (P-22) and `src/decompress.rs` resolved
  them: C pastes `W32_PFN_##func`/`fn_##func` and stringifies the name, so both are parameters
  here.  `CDECL` is empty on this target's ABI, so the pointer type is the plain `extern "C"` one.
- **`dynlib_*` comes from the mirror that owns it.**  C 864-941 names `dynlib_handle_ptr`,
  `dynlib_open`, `dynlib_error`, `dynlib_sym` and `dynlib_addr`, and this snapshot's `treesit.c`
  carries **no include that declares them** - a real defect of the reference text (the region is
  never compiled in the frozen configuration, so C tolerates it).  The port includes
  `include/dynlib.rs` under the file gate; reported as finding (a) below.
- **Uninitialized locals are zeroed** (PORTING.md 1.5): `override_name`/`override_c_name` (C 826-827)
  and `handle`/`error`/`sym` are C's uninitialized declarations, and C 786's `char ext[16]` is
  `std::mem::zeroed ()`; C's own later assignments stay as statements.
- **Two spellings the flip reading corrected** (both were defects of this round's first text, not
  of C): `BUF_Z` is a `macro_rules!` in the `buffer.h` mirror, so the call site is `BUF_Z! (buf)`
  (C's is a plain call - the mirror's P-37 note says the call site spells it), and `xpalloc`'s size
  argument is a `ptrdiff_t`, so `size_of::<treesit_lang_cache_entry> () as ptrdiff_t`.
- **`(*ranges).end_byte == -1` is `u32::MAX`.**  C compares a `uint32_t` with the `int` -1, i.e.
  the all-ones value; a bare `-1` is not a `u32` expression in Rust.
- **`%"PRIu32"` in C 609's `printf` is the concatenated literal** `c" [%u, %u)"`, the value
  `lib/include/inttypes.in.rs` 201 measures for the frozen `<inttypes.h>`.
- **Named gaps this unit does not close** (listed in `## Decisions`): the `TS*` types and the two
  api.h version macros, `XTS_PARSER`/`Lisp_TS_Parser`, and the `line-drift-tolerance.tsv` row.

## Findings

(a) **The reference's own text names five `dynlib_*` names with no declaration in scope.**
    `emacs/src/treesit.c` 864-941 uses `dynlib_handle_ptr` and the four functions, and the file's
    five includes (`config.h`, `lisp.h`, `buffer.h`, `coding.h`, `treesit.h`) reach `dynlib.h`
    through none of them (`grep -n dynlib emacs/src/lisp.h` is empty; `treesit.h` 25 includes
    `<tree_sitter/api.h>`, `lisp.h`, `buffer.h`).  A tree-sitter configuration would need an
    `#include "dynlib.h"`; the frozen configuration never compiles the region, which is why the
    snapshot tolerates it.  The port takes the names from the owning mirror and records this here
    rather than pretending C declared them.

## Evidence

**① The frozen configuration (`build-tu`), unchanged by this round.**
`cd femacs && CARGO_TARGET_DIR=${LANE}/cargo cargo run -q -p xtask -- build-tu src/treesit.c`
-> `warning: 2 warnings emitted` (both in mirror copies: `include/lisp.rs` 3934 and
`lib/include/stdbit.in.rs` 644, "unused doc comment" - the same two the empty-unit revision
printed), then `built src/treesit.rs -> ../build/femacs/treesit.o`, **exit 0**.
`build/femacs/treesit.o` is **2,656 bytes**, and `nm -g` lists exactly
`T _Ftreesit_available_p`, `T _syms_of_treesit` plus the mirror's own inline helpers
(`xfree_wrapper`, `flush_stack_barrier`, `cons_to_integer`) with `U __setjmp`, `U _cons_to_signed`,
`U _cons_to_unsigned`, `U _defsubr`, `U _xfree`; `nm -g … | grep -c "ts_\|TREESIT"` -> **0**.
No gated item and no gated mirror name reaches the frozen object.
(*The pre-round object's byte size is not recorded in the earlier ledger, so "unchanged" is read
from the symbol set and the 2-warning profile, both of which the 48-line revision produced too.*)

**② The line count.**  `wc -l src/treesit.rs` -> **1216** (was 48); the transcribed C span is
C 29-946, 918 lines of C.

**③ `extract-abi --check` fails on the ratchet, and the row that fixes it is not this lane's.**
`python3 tools/extract-abi.py --check emacs/src/treesit.c src/treesit.rs` ->
`- line count drifted 2194.3%: C 53 live (of 5634) vs Rust 1216` / `FAIL …`, **exit 1**.
`docs/ports/line-drift-tolerance.tsv` 145 is
`treesit	0.35	C 53 live (of 5634) vs the port's 69: … (P-08).	live` - a `live`-basis row whose
prose also says the dead region is "recorded, not transcribed".  Both halves are now out of date:
the region *is* being transcribed (this is P-86's shape) and the baseline has to become the file's
**total** 5,634 lines (the `decompress`/`lcms` rows' basis), tolerance ≤ 0.35, which the port's
1,216 lines (21.6%) passes.  P-86 ⑩ records this exact wall for the first P-86 file: the tolerance
table is the integrator's, not the porter's.  `tools/check-port-lines.py` agrees from the other
side: `stale    src/treesit.c: no longer under the threshold` (`docs/baseline/port-lines-pending.tsv`
68 still says `5634	48	563	known-unported target (plan batch F)`).

**④ The two records' layout, measured (not remembered).**  No `<tree_sitter/api.h>` exists in the
reference tree and none is installed on this host, but the two records this unit defines are made of
pointers and words, so their ABI is measurable with stand-ins:

```
$ clang -o ${LANE}/layout-probe ${LANE}/layout-probe.c && ${LANE}/layout-probe
sizeof(Lisp_Object)=8 alignof=8
sizeof(treesit_loaded_lang)=16 alignof=8 lang=0 filename=8
sizeof(treesit_lang_cache_entry)=24 alignof=8 symbol=0 lang=8
```
(`typedef struct TSLanguage TSLanguage; typedef intptr_t Lisp_Object;` - the incomplete api.h type
if the header were present, and the port's word-sized tagged pointer, `src/include/lisp.rs`
82-83.)  Those four numbers are the `const _: () = assert! (…)` items in the port
(`std::mem::size_of`/`offset_of`), which the flip would check if the arm could be compiled.
The temporary directory is cleaned up, so the probe's source is kept here:

```c
/* /tmp/<prefix>/layout-probe.c - the two records src/treesit.c defines in its first slice.  */
#include <stdio.h>
#include <stddef.h>
#include <stdint.h>

typedef struct TSLanguage TSLanguage;   /* the incomplete type <tree_sitter/api.h> declares */
typedef intptr_t Lisp_Object;            /* the port's word-sized tagged pointer */

struct treesit_loaded_lang { TSLanguage *lang; const char *filename; };
struct treesit_lang_cache_entry { Lisp_Object symbol; struct treesit_loaded_lang lang; };

int main (void)
{
  printf ("sizeof(Lisp_Object)=%zu alignof=%zu\n", sizeof (Lisp_Object), _Alignof (Lisp_Object));
  printf ("sizeof(treesit_loaded_lang)=%zu alignof=%zu lang=%zu filename=%zu\n",
          sizeof (struct treesit_loaded_lang), _Alignof (struct treesit_loaded_lang),
          offsetof (struct treesit_loaded_lang, lang),
          offsetof (struct treesit_loaded_lang, filename));
  printf ("sizeof(treesit_lang_cache_entry)=%zu alignof=%zu symbol=%zu lang=%zu\n",
          sizeof (struct treesit_lang_cache_entry), _Alignof (struct treesit_lang_cache_entry),
          offsetof (struct treesit_lang_cache_entry, symbol),
          offsetof (struct treesit_lang_cache_entry, lang));
  return 0;
}
```

**⑤ The arm flip: `text-only`, and it is two independent blockers, not one.**
`which pkg-config` -> empty; `ls /usr/local/lib/libtree-sitter* /opt/homebrew/lib/libtree-sitter*
/opt/homebrew/include/tree_sitter` -> all "No such file or directory"; `find / -name
'libtree-sitter*'` and `find / -name api.h -path '*tree_sitter*'` (depth-limited sweeps) -> nothing.
So **the missing toolchain is libtree-sitter itself, headers and library**: no `TS*` declaration and
no `ts_*` symbol can be had, and no clang reading of `<tree_sitter/api.h>` is possible (④'s
stand-ins are the most that can be measured).
`cargo run -q -p xtask -- probe src/treesit.rs ${LANE}/flip2.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_tree_sitter"'` -> **exit 1, 34 errors, 2 warnings**, and the
errors by primary location are: **25 in this unit's own text** (`build/probes/treesit/_root.rs`) and
**9 in the `include/buffer.rs` copy** (`_Users_…_include_buffer_rs.rs`).
The 25 are *all* unresolved names, and every one of them is on the named-gap list: `TSLanguage` x6,
`TSRange` x2, `TSPoint` x2, `TSParser` x1, `Lisp_TS_Parser` x1, `XTS_PARSER` x3,
`TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION` x2, `TREE_SITTER_LANGUAGE_VERSION` x2,
`ts_set_allocator`/`ts_parser_included_ranges`/`ts_language_version`/`ts_parser_new`/
`ts_parser_set_language`/`ts_parser_delete` x6.  **There is no syntax error, no type error and no
`E0428` in this unit's text.**
The 9 mirror-copy errors are the `buffer.rs`/`composite.rs` copies' own unresolved names
(`UNIBYTE_TO_CHAR`, `STRING_CHAR`, `string_char_and_length`, `BYTES_BY_CHAR_HEAD`,
`raw_prev_char_len`) plus one `E0080` at `include/buffer.rs` 1222:
`const _: () = core::assert! (!cfg! (feature = "have_tree_sitter") || HAVE_TREE_SITTER);` - P-51's
binding assert, i.e. **a bare `--cfg` cannot compile a carried arm by design**; that reading
belongs to `xtask dead-arms`, which this task's build budget excludes (the same wall
`docs/ports/decompress.md` ③ records).  This is *not* the same reading as ①.
**The first flip run is why ⑤ exists at all**: it read 39 errors and two of them were this unit's own
defects (`E0423 BUF_Z`, `E0308 xpalloc`) and one was `E0428 syms_of_treesit`; all three were fixed
before the reading above (they are the `## Deviations` entries).

**⑥ The two checks the task names.**
`python3 tools/check-declarations.py` -> **exit 0**, 0 `FAIL`/`STALE` lines (no copied declaration
prose reaches the unit or this ledger; the walk read 682 files).
`python3 tools/check-dead-file-coverage.py` ->
`info docs/ports/treesit.md: 31 range row(s) cover src/treesit.c (5634 lines; 0 blank line(s) between
blocks need no row)` / `25 ledger(s) bound (a C range table), 0 failure(s)` -> **0 failures**.  The
same run also prints the line `31 range row(s) cover src/treesit.h` for this table: the tool resolves
`src/include/treesit.rs` to the *stem*'s ledger, and the header's own
`docs/ports/include-treesit.md` carries no range table, so the unit's table answers for it too - the
pre-existing behaviour for every `.c`/`.h` pair (`w32term.c` and `w32term.h` both print
`docs/ports/w32term.md`), recorded here so the reading is not read as a claim about the header.

**⑦ The host's C side, for the record.**  The reference's own text was not compiled this round:
`clang` was used only for ④'s stand-in probe.  Nothing in this ledger is a reading of a
tree-sitter-enabled build, on this host or any other.

## P430 complete macOS feature construction

Current baseline: `src/treesit.rs:1-1216` ends the gated body after original
C946; `src/include/treesit.rs:1-32` omits the gated layouts/accessors. The
existing fallback `treesit-available-p` always returns nil. Original line/column
tests therefore skip three more cases than the installed stable comparator.

Target flow: buffer edit/case/transpose -> line/column caches and per-parser
TSInputEdit/timestamp -> lazy ensure-parsed -> TS parser/tree -> rooted Lisp
node/query -> predicate/navigation API. Nodes retain parsers; parsers retain
associated buffers. Query source/language remain Lisp roots; external query and
cursor handles are nullable and reset for dumping, then rebuilt lazily. Parser
input payloads/callbacks must always belong to the exact live parser/buffer.

- [x] Mirror 0.27.1 SDK ABI and original header parser/node/query layouts,
  predicates, accessors and external functions; one shared owner for all callers.
- [x] Port C948-1044 language/grammar entrypoints; C1045-2057 line/column, edit
  recording, visible-region synchronization, input callback and lazy parsing.
- [x] Port C2058-3487 object allocation/destruction, parser/node APIs and errors.
- [x] Port C3488-5292 query construction, predicates/capture and full navigation.
- [x] Port gated syms initialization, variables and error hierarchy; availability
  reports the real selected feature/library. Preserve feature-disabled behavior.
- [x] Restore every original editor/GC/printing/dumping hook; remove shadow
  stand-in layouts only with a traceable declaration-consolidation record.
- [x] Capture SDK header/version/library/license before linking; enable both
  macOS frontends and package relative runtime dependencies, with complete
  product receipts and source identities. Preserve unrelated runtime work.

After all business work: measured SDK/Lisp layout/function contracts, fresh
normal/checking Cocoa/terminal builds and strict app signatures; unchanged
stable original Tree-sitter file plus real owned grammar inputs on reference
and candidates, actual edit/narrow/indirect-buffer/node/query/GC/dump checks,
original IDs/timings/skips/failures retained. Then critical protection cases,
independent review, coherent source delivery and consumed-artifact cleanup.
No build, test or acceptance has yet been performed for P430 as of the business-writing checkpoint; later measured results must be recorded separately.

P430 source selection refinement: use pinned stable 31.1 by symbol, whose
missing public group starts at C898, not fd7 C948. The two header versions are
byte-identical. The inactive fd7 language cache is excluded: stable grammar
lookup must observe changes to configured paths/overrides; stable calloc wrapper
is restored. See CLN132 source rebase/consolidation record. No test was run.

P430 business-writing update: all 132 stable definition names, including all
58 original Lisp entrypoints, now have source bodies. The shared header owns
SDK/parser/node/query declarations; buffer lifecycle, insertion/deletion,
substitution/translation/transposition, case conversion, GC, print and dump
consumers are connected. Both target configurations and independent SDK
capture/link/license packaging have been written. These are construction
facts only: no compiler, ABI, original test or product acceptance result has
yet been obtained for this new component. The old disabled-feature readings
above are historical, not measurements of this new enabled candidate.


P430 measured integration update: four fresh complete-workspace native products
and two strict Cocoa signatures passed; 123 actual SDK/Lisp/buffer observations,
55 function contracts and three callback contracts agree. All unchanged original
40 cases pass on five products (200, no skips); 15 unique lifecycle/GC cases and
five two-process query image restores pass. Two later configuration guards pass.
First failures remain recorded; no original assertion was modified. The earlier
construction-only statements are historical checkpoints. Independent review,
coherent prerequisite source delivery and consumed-copy cleanup remain pending;
this is not whole G3 or clean committed-checkout acceptance. See
`maintenance/releases/2026-10-29-macos-treesit.md`.


P430 independent review: 20 actual read-only source/evidence checks accepted
the limited implementation and measured products, no concrete failure found
in that scope. All 58 original DEFUN names match; the earlier 132-name
construction count is not a Rust function-token count or acceptance metric.
Coherent runtime/TLS/LCMS/owned-copy prerequisite source delivery remains pending;
do not stage all WIP or publish a half-wired feature. Goal remains active 1/7.


Historical command transcripts above use `${LANE}` for their old disposable
artifact root. This is a documentation path normalization, not a new execution
or a change to the retained measurements. Current product builds use the owned
entrypoints and dependency instructions in `build-support/README.md`.
