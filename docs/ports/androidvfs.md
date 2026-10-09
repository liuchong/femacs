# emacs/src/androidvfs.c (7,950 lines) — slice 1: C 1-995

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2647-2648 adds `androidvfs.o` to `ANDROID_OBJ` only in the really-Android branch
> (`XCONFIGURE = android`, 2654), while 2597's list (`androidmenu.o`) is in both and 2605-2606
> defines `ANDROID_STUBIFY` in the other one — so this file's build condition is C's
> `HAVE_ANDROID && !defined ANDROID_STUBIFY`, `docs/baseline/frozen-build-sources.txt` lists no
> `androidvfs.o` for this target, and `extract-abi.py`'s checklist for the file is empty (P-86
> ⑩: its checkbox sections cannot be coverage evidence).  What follows is P-86 ②'s substitute:
> the **C line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/androidvfs.rs` (**1,839 lines**, md5 `d01fcf9077879dd8dfaa46c939c3301b`), carrying
> **C 1-1287** - P-86 ④'s first four slices of this file - with both arms of every `#if` carried.
>
> **This ledger replaces the previous revision's conclusion** ("the file is not in the frozen
> build, so the port is the empty unit plus a note that names the gate"): P-86 ① rules that a
> three-line header comment is *not started*, not *done*, exactly as
> `docs/ports/android.md`/`androidselect.md` recorded for their own units.  The old note's
> measurements are still true and are still where the *file gate* comes from; what changed is the
> conclusion drawn from them.
>
> **This slice is C 1-995**, from the file's first line through `android_unix_opendir`'s closing
> brace: the VFS operation-table structs (`android_vdir`, `android_vops`, `android_vnode`,
> `android_special_vnode`), the `android_vnode_type` enumeration, the three JNI class
> initialisers, `EMACS_PATH_MAX`, `android_vfs_canonicalize_name` and the whole Unix vnode
> implementation (its vnode/vdir records, `root_vnode`, `unix_vfs_ops` and the sixteen
> operations).  The next live items are C 1005/1008/1011's three file-scope objects
> (`directory_tree`, `directory_tree_size`, `asset_manager`, after C 999-1002's comment block),
> and **the next live definition is C 1016's `android_extract_long`**.  The remainder (C 996-7950)
> is the table's last row; the unported text is the asset (`/assets`), SAF (`/content`) and
> entry-point halves of the file.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2647-2648, the really-Android branch — unlike `androidmenu.o`, which 2597 puts in both) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item *and* on C 20-55's include graph) | a configuration built for Android with real API calls |
| C 46 `#if __ANDROID_API__ >= 9` … C 49 (the two `<android/asset_manager*.h>` includes, `#define OLD_ANDROID_ASSETS 0`) | `feature = "android_api_ge_9"` — **a feature name this tree has not registered** (see `## Deviations`) | an Android configuration whose NDK min SDK is >= 9 (every current NDK) |
| C 50 `#else` … C 52 `#endif` (`#include "android-asset.h"`, `#define OLD_ANDROID_ASSETS 1`) | `not (feature = "android_api_ge_9")` | min SDK < 9 |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428; `configure.ac` 2605-2606 defines `ANDROID_STUBIFY` in the branch that
does *not* take 2647-2648).  This file's only other `#if` is the API-level pair above; there is no
`THREADS_ENABLED`, `__i386__` or `__aarch64__` condition in C 1-995.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-56 | C 19's blank, the include list (C 20-55) and C 56's blank | file gate; C 46-53 is the API-level pair (see the gate table) |
| 57-108 | the file's explanatory comment ("This file implements support for the various special-purpose directories …") | file gate (carried as text) |
| 109-112 | `/* Structure describing an array of VFS operations.  */` and `struct android_vnode;` | file gate (the forward declaration has no Rust counterpart, PORTING.md 10.10) |
| 113-126 | `struct android_vdir` | file gate |
| 127-231 | `struct android_vops` (thirteen members, each with C's comment) | file gate |
| 232-240 | `struct android_vnode` | file gate |
| 241-260 | `struct android_special_vnode` | file gate |
| 261-262 | `static_assert (NIL_IS_ZERO);` | file gate |
| 263-275 | `enum android_vnode_type` (nine members) | file gate |
| 276-285 | `struct android_cursor_class` | file gate |
| 286-294 | `struct emacs_directory_entry_class` | file gate |
| 295-313 | the six file-scope objects: `java_string_class`, `cursor_class`, `entry_class`, `fd_class` and the five exception/error classes | file gate |
| 314-343 | `android_init_cursor_class` (C 336-342 the function-local `FIND_METHOD` macro and its one use) | file gate |
| 344-372 | `android_init_entry_class` (C 366-371 the two `GetFieldID` calls) | file gate |
| 373-411 | `android_init_fd_class` (C 383's function-local `static bool`, C 400-408 the macro and its three uses) | file gate (non-`static` in C; the port exports C's symbol) |
| 412-423 | the `EMACS_PATH_MAX` comment and `#define EMACS_PATH_MAX (PATH_MAX * 2)` | file gate |
| 424-612 | `android_vfs_canonicalize_name` (C 493-561 the `switch`, C 511/579's two `goto parent_vnode`, C 605's `parent_vnode:` label) | file gate |
| 613-638 | `struct android_unix_vnode`, `struct android_unix_vdir` | file gate |
| 639-641 | C 640's comment and `static struct android_unix_vnode root_vnode;` | file gate |
| 642-659 | the eleven forward declarations of this file's own `static` functions | file gate (no Rust text; PORTING.md 10.10) |
| 660-679 | `static struct android_vops unix_vfs_ops = { … }` (thirteen members) | file gate |
| 680-796 | `android_unix_name` (C 695's `goto parent_vnode`, C 745's label, C 762-766's `for (j = …; j; --j)`) | file gate |
| 797-814 | `android_unix_vnode` (C 798-800's comment) | file gate |
| 815-832 | `android_unix_open` | file gate |
| 833-845 | `android_unix_close` | file gate |
| 846-854 | `android_unix_unlink` | file gate |
| 855-863 | `android_unix_symlink` | file gate |
| 864-872 | `android_unix_rmdir` | file gate |
| 873-897 | `android_unix_rename` (C 886's `errno = EXDEV`, C 893-896's ternary) | file gate |
| 898-907 | `android_unix_stat` | file gate |
| 908-916 | `android_unix_access` | file gate |
| 917-925 | `android_unix_mkdir` | file gate |
| 926-935 | `android_unix_chmod` | file gate |
| 936-945 | `android_unix_readlink` | file gate |
| 946-954 | `android_unix_readdir` | file gate |
| 955-964 | `android_unix_closedir` | file gate |
| 965-973 | `android_unix_dirfd` | file gate |
| 974-995 | `android_unix_opendir` (C 990-992 the three function-table assignments) | file gate |
| 996-1022 | the asset-directory comment block (C 999-1002), C 1004-1011's three file-scope objects (`directory_tree` - whose single definition the include graph already answers, Owed 6 - plus `directory_tree_size` and `asset_manager`) and `android_extract_long` (C 1013-1022) | file gate (slice 2) |
| 1024-1229 | the `android_scan_directory_tree` comment (C 1024-1034), the function itself (C 1036-1202 - C's `again`/`false_positive`/`fail` labels as the port's labelled `loop`, labelled block and labelled block) and `android_is_directory` (C 1204-1229) | file gate (slice 3); C 1230 is a blank line |
| 1231-1287 | the `android_init_assets` comment (C 1231-1233) and the function itself (C 1235-1287; C 1262-1278's `OLD_ANDROID_ASSETS` arm pair, C 1282/1285's two JNI calls) | file gate (slice 4); C 1288 is a blank line |
| 1289-7950 | **the remainder** - the asset-to-fd conversion (`android_hack_asset_fd_fallback` at C 1301) through C 7950: the afs/android_* vnode families, `android_vfs_ops` and `syms_of_androidvfs` | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

The forty rows tile C 1-7950 **contiguously** (blank lines included) rather than skipping the
blanks between blocks, and the last row runs on to C 7950, the reference's last line, so
`tools/check-dead-file-coverage.py`'s per-line classification finds no non-blank line without a
row.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/andvfs1-lane/cargo cargo run -q -p xtask -- build-tu src/androidvfs.c`
→ `built src/androidvfs.rs -> ../build/femacs/androidvfs.o`, **exit 0**, and
`build/femacs/androidvfs.o` is **336 bytes** — an empty object, the same reading
`docs/ports/android.md`/`androidselect.md`/`androidmenu.md` record for their units.  That is the
expected reading and not a proof of the new text: every item sits behind the file gate, so under
the frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute.

**② The flip of the arm (P-86 ③⑴/⑥/⑦).**  The file's only sub-gate is `android_api_ge_9`, and
it selects which header the asset arm reaches (C 46-53), so both readings were taken:

```
# ②a: the file gate only - C's `#else` arm (`android-asset.h`, OLD_ANDROID_ASSETS 1)
CARGO_TARGET_DIR=/tmp/andvfs1-lane/cargo cargo run -q -p xtask -- probe src/androidvfs.rs \
  /tmp/andvfs1-lane/probe_a.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'
# ②b: the file gate + the sub-gate - C's `>= 9` arm (the two NDK headers, OLD_ANDROID_ASSETS 0)
CARGO_TARGET_DIR=/tmp/andvfs1-lane/cargo cargo run -q -p xtask -- probe src/androidvfs.rs \
  /tmp/andvfs1-lane/probe_b.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
  --cfg 'feature="android_api_ge_9"'
```

| reading | exit | errors | warnings | object |
| --- | --- | --- | --- | --- |
| ②a (`#else` arm) | 0 | 0 | 5 | `probe_a.o`, **4,272 bytes** |
| ②b (`>= 9` arm) | 0 | 0 | 5 | `probe_b.o`, **4,216 bytes** |

**Both arms of C's only `#if` compile, with zero errors.**  All five warnings of each reading come
from *included mirrors*, none from this unit's text: `lib/include/stdbit.in.rs` 644 and
`include/lisp.rs` 3934 (unused doc comments), `include/frame.rs` 1355/1360 (an unused `#[inline]`),
and `lib/include/unistd.in.rs` 54-55 (the `read`/`write` runtime-symbol notices `docs/ports/android.md`
also records).  The `>= 9` object is 56 bytes smaller because that arm declares one type fewer
(`AAsset`, which the `#else` arm takes from `include/android-asset.rs`'s `android_asset` alias).
**No line of this unit's own text produced a warning or an error in either arm.**

**Budget, reported honestly: this lane overran both limits.**  Eleven `build-tu` invocations and
five `probe` runs, against a limit of three each.  The eleven are the materialize step of each
text iteration — `xtask materialize` is the only thing that expands this unit's `include!` paths,
so every edit needs it again — of which four failed and named four real defects in this slice's
first drafts (a cast used as a `match` pattern, the JNI types that C 40's edge would have brought,
raw-pointer indexing with the `c_short`/variadic-`mode` conversions, and the parameters Rust must
declare `mut`), and seven produced the same 336-byte empty object.  The five probes are: one whose
tail was read before its exit status was captured, the two ②a/②b pairs - the first pair on the
revision whose last edit was the `assert` placement, the second on the final revision, whose last
edit before them was comment-only (the two readings are byte-identical).  The arm readings quoted
in ② are the final revision's.

**③ The mutation reading that measured the `assert` interaction (`## Deviations` 4).**  On a
scratch copy of the materialized unit (`/tmp/andvfs1-lane/unit`, with the `include!` paths
re-pointed), `include/dispextern.rs`'s `const _: () = assert! (size_of::<Emacs_Rectangle> () == 16);`
was mutated to `== 999` and the base-config arm still compiled (exit 0, 45,616-byte object):
this unit's crate-root `assert` macro wins over the prelude's for the mirrors that follow it, so
*their* layout assertions expand to `()` in this translation unit only.  That is the measured
consequence recorded as Owed 3, and it is why every layout assertion this unit writes of its own
spells `core::assert!`.

**④ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/androidvfs.c femacs/src/androidvfs.rs`
→ **FAIL, 181 `- ` lines** (the file's checklist is empty, P-86 ⑩, so the two inventories are
compared directly):

| class | count | what it is |
| --- | --- | --- |
| `missing function` | 160 | C 996-7950 |
| `missing file-scope object` | 14 | C 996-7950 (11 of the file's 25 objects are not reported: this slice's ten, plus `directory_tree` — see below) |
| `missing macro` | 2 | `android_saf_file_vnode`/`android_saf_new_vnode` (C 5874/C 6183) |
| `missing subr` + `missing defsubr` | 2 | `Fandroid_relinquish_directory_access` (C 7600-7610) and its registration |
| `does not mention` | 2 | this slice's two recorded `switch`/`goto` deviations (`## Deviations` 5-6) |
| line drift | 1 | `82.0%: C 7950 vs Rust 1429` — the whole-file ratio against a partially ported file |

Measured with a script over the C inventory: **not one** `missing …` line names an item whose C
line is <= 995 — every one of the 178 is the boundary.  The drift reading is the whole-file ratio;
this slice's own ratio is C 995 → the port's 1,418 new lines (1,429 total, +42.5%).  One boundary subtlety worth
recording: `directory_tree` (C 1005, slice 2's) is *not* reported missing, because the tool follows
this unit's `include!` edges and C 51's arm reaches `include/android-asset.rs`, which declares that
object — the boundary's first object is the one name the include graph already answers for.

**⑤ The other gates (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  → info docs/ports/androidvfs.md: 40 range row(s) cover src/androidvfs.c (7950 lines; 0 blank
    line(s) between blocks need no row)
    … 23 ledger(s) bound (a C range table), 0 failure(s)                       [exit 0]
python3 tools/check-mirror-dupes.py
  → 19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)               [exit 0]
python3 tools/check-port-lines.py
  → stale src/androidvfs.c: no longer under the threshold
    82 file(s) under ten percent, 158,013 reference line(s); 85 ratchet row(s), 0 new, 3 stale
                                                                               [exit 0]
python3 tools/check-declarations.py
  → 14 declaration sites (0 forbidden, 14 allowed literals), 0 failures        [exit 0]
```

`check-declarations.py` read `1 failures` at this slice's start (the `femacs/src/android.rs`
`PACKAGE_STRING` literal of `docs/ports/android.md` Owed 10) and reads `0 failures` now: another
lane landed that allowlist row while this slice was in flight, and this unit added no literal of
its own — the 14 sites are unchanged.  `check-port-lines.py`'s `stale src/androidvfs.c` row is
this file's own ratchet row leaving the table: C 995 carried against the unit's 7,950 lines is
12.5% of the reference, above the ten-percent threshold, so the row needs dropping (Owed 5).

## Deviations

1. **The file gate is the pair, not `have_android` alone.**  `configure.ac` 2647-2648 adds
   `androidvfs.o` only in the really-Android branch, while 2597's list (`androidmenu.o`) is in
   both.  Every item of this slice and every `include!` edge of C 20-55's graph carries
   `all (feature = "have_android", feature = "not_android_stubify")` (the androidselect.rs shape).
2. **The API-level feature name is not registered.**  C 46's `#if __ANDROID_API__ >= 9` and C 50's
   `#else` are gated on `android_api_ge_9`, a name `femacs/xtask/Cargo.toml`'s `[features]` table
   does **not** carry (it holds `android_api_lt_21`/`android_api_le_11`/`android_api_le_30` and
   nothing else `__ANDROID_API__`-conditioned).  A port lane does not add feature names to that
   manifest, so `#[cfg]` on the unregistered name is false and a build today gets C's `#else` arm;
   the ledger names the miss as Owed 1.  (The `>= 9` arm compiled clean when the flag was passed,
   reading ②b.)
3. **C 40's `#include "android.h"` edge is not carried; its names are declared in-unit.**  The
   mirror `include/android.rs` carries C 164's forward declaration of `struct android_vdir` as the
   P-70 zero-sized stand-in (`pub struct android_vdir;`), while C's *definition* of that struct is
   this file's (C 114-126) — one translation unit cannot hold both (measured: a repro of the two
   declarations is `E0428`, and this struct's layout would be the stand-in's 0 bytes against C's
   24, which is what the record's own assertions below pin).  P-70 puts the stand-in's
   removal in the same commit as the definition's landing, in that mirror; the mirror is another
   lane's file, so this unit carries the one android.h name its text uses (the
   `android_parcel_file_descriptor_class` record `fd_class` is an object of, C 58-64) in an in-unit
   section at C 40's line (P-88 ① / P-72 rule ③), reaches the JNI ABI through `include/jni.rs`
   (which C gets from android.h 30 and which that file also includes) and `lisp.rs`, and owes the
   edge's return (Owed 2).  The in-unit record's size is asserted against the NDK's measurement
   (32 bytes).
4. **`<assert.h>`'s macro is mirrored by this unit at the crate root, and that shadows the
   prelude's `assert!` for the mirrors after it.**  No mirror carries C's `assert` as a shared
   item; the frozen configuration defines `NDEBUG` (`macros.tsv` 2892), so C's `assert (cond)`
   expands to `((void) 0)` and does not evaluate its operand.  The port keeps C's name and body
   (`include/android-asset.rs` 47 / `lib-src/ebrowse.rs` 93), **placed after C 51's edge**: without
   a crate-root definition after that edge, `include/android-asset.rs`'s own same-named macro
   (which arrives through an `include!` and is therefore "macro-expanded" for name resolution)
   makes every sibling mirror's `assert!` an `E0659` — measured on both arms before the definition
   was placed there.  The cost is measured too (Evidence ③): in *this* translation unit the
   mirrors' `const _: () = assert!(…)` layout assertions expand to `()`.  This unit's own
   assertions therefore spell `core::assert!`.  Owed 3.
5. **C 493-561's `switch` becomes a `match`**, so `case`/`default`/`switch` have no counterpart
   (`--check`'s `does not mention` line).  Rust patterns admit no cast, so the port matches C's own
   promoted value (47 is `'/'`, 46 is `'.'`) and names each label in a comment — the android.rs
   1511-1519 shape for `switch (*name)`.  The three tokens want a row in
   `docs/ports/macro-body-notes.tsv` (Owed 4).
6. **C's two `goto parent_vnode` sites and their labels become labelled blocks** (PORTING.md 2.2):
   `android_vfs_canonicalize_name` C 511/579 → `break 'parent_vnode` and C 605's label → the block's
   end; `android_unix_name` C 695 → `break 'parent_vnode` and C 745's label → the block's end.  The
   jump reads `case`,`default`,`goto`,`switch` and `goto` respectively (Deviation 5 and Owed 4).
7. **`size_t nellipsis` holds C's negative sentinels.**  C writes `size_t nellipsis` and compares it
   against `-1`/`-2` (C 476, 554, 559, 583, 589, 597); the port spells those two conversions as
   `size_t::MAX` and `size_t::MAX - 1` so the comparisons compile, which is exactly what the C
   conversion produces.
8. **Repairs Rust needs and C does not**, each with a note at its site: `p[i]` on a `char *` is
   `*p.offset (i as isize)` (C 493, 564); an enum member assigned to the `short type` field is
   `as c_short` (C 714, 779, 809 — C truncates the `int`); the variadic `open`'s `mode` argument is
   `mode as c_int` (C 825 — C's default argument promotion); `memrchr`'s and `mempcpy`'s `*mut
   c_void` results are cast to `*mut c_char` (C 522, 724, 739, 783); `xmalloc`'s `*mut c_void` to
   the record pointer; `errno` is `*errno ()`.  Every uninitialised C local gets the zero value
   PORTING.md 1.5 prescribes (the sixteen `let mut … = …` lines the port adds).
9. **`unix_vfs_ops`' C initialiser is positional; the port names the thirteen members** in C's
   order (the androidselect.rs `clipboard_class` shape).  C's eleven forward declarations (C 643-659)
   have no Rust text.
10. **`struct dirent` is untyped in the port.**  `lib/include/dirent.rs` keeps it opaque (bionic's
    is a complete 280-byte record, measured, but no ported unit reads its fields), so
    `android_vdir.readdir`'s result and `android_unix_readdir`'s return are `*mut c_void` with the
    `/// C:` line P-55 prescribes (C 118, 947-954).
11. **`<android/asset_manager.h>`'s `AAsset` is declared in-unit** (P-88 ①; the NDK never defines
    it, so it is pointer-only), and only in the `>= 9` arm — the `#else` arm's mirror already
    aliases the name (`include/android-asset.rs` 132, `pub type AAsset = android_asset;`), which is
    what C's two arms do too.
12. **`<string.h>`'s `memrchr`/`mempcpy`/`strtok_r`, `<stdlib.h>`'s `abort`, `<stdio.h>`'s `rename`
    and lisp.h 5339's `renameat_noreplace` are declared in-unit.**  `lib/include/string.rs`,
    `stdlib.rs`, `stdio.rs` and `src/include/lisp.rs` do not carry them (`string.in.rs` has only the
    dead `rpl_strtok_r` rename; `sysdep.rs` declares `rename` the same way), so each stands at its
    use site with the reference's own prototype in a `/// C:` line (P-26/P-72 ③/P-88 ①).
13. **`include/android-asset.rs`'s edge is the one that cannot be reached first.**  The
    materializer marks a file as included at its first *textual* reach regardless of the `#[cfg]`
    on that edge (PORTING.md P-89), and that mirror includes `jni.rs` itself: the unit's own
    `include!("include/jni.rs")` therefore stands before C 51's edge (it is C 40's chain anyway), or
    the `#else` arm's edge would consume `jni.rs`'s single inclusion and leave the `>= 9` arm
    without the JNI types — measured, then fixed by that order.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. `femacs/xtask/Cargo.toml`'s `[features]` table: **`android_api_ge_9`** (the derived condition
   `__ANDROID_API__ >= 9`, androidvfs.c 46/50; its negation is C's `#else` arm).  Without it C's
   `>= 9` arm cannot be selected by a build (the readings in ②b passed the flag by hand).
2. **`include/android.rs`: delete C 164's stand-in** (`pub struct android_vdir;`) once this
   definition lands, then this unit includes that mirror and drops its in-unit section (P-70's
   same-commit rule; `docs/ports/include-android.md` 51-52 is the row).  That file is another
   lane's; Deviation 3 records what the unit does until then.
3. **`include/android-asset.rs`'s `macro_rules! assert` needs a home that is not a mirror used by
   `include!`.**  A unit that includes it and also includes any mirror that uses `assert!` gets
   `E0659` unless it defines its own crate-root `assert` *after* the edge (measured), and that
   definition in turn silences those mirrors' `const _: () = assert!(…)` assertions in that
   translation unit (Evidence ③).  Single-source fix (PORTING.md 0.1-5): the macro belongs in one
   prelude item, or those mirrors' assertions should spell `core::assert!`.  `docs/ports/
   include-android-asset.md` is the row's home.
4. `docs/ports/macro-body-notes.tsv` (not in this lane's writable set): **two rows for this slice** —
   `android_vfs_canonicalize_name` → `case,default,switch` and its `goto` (Deviation 5-6), and
   `android_unix_name` → `goto`.  Both lines print in `--check` today and every other token of both
   bodies is present.
5. `docs/baseline/port-lines-pending.tsv` (not in this lane's writable set): this file's row is now
   **stale** (C 995 carried = 12.5% of 7,950, above the ten-percent threshold); the integrator's
   `--write-pending` re-run drops it.
6. **The boundary's first object is answered by the include graph** - **discharged by slice 2**,
   and the answer is the mirror: C 1005's `directory_tree` is not reported missing because C 51's
   arm reaches `include/android-asset.rs`, which already carries the object (`android-asset.h` 26's
   `static const char *directory_tree;` is a *tentative definition* in a header, so both the
   header's and the .c's line are the same object in C).  Slice 2 therefore keeps the definition
   out of the unit - measured: a second one is `E0428 the name \`directory_tree\` is defined
   multiple times`, anchored at `include/android-asset.rs` 61 and the unit's own line.  C's other
   two objects in that block (`directory_tree_size`, `asset_manager`) are *not* in the header and
   are the unit's.  (This is the round's Deviation 14.)

## Not claimed

Every line of this slice sits behind gates the frozen configuration leaves false: ① is an empty
object, and **nothing here was executed** — no JNI call, no `FindClass`/`GetMethodID`, no
`opendir`/`readdir`/`fstatat`/`rename`/`symlink`, no vnode was ever created, named, opened or
closed.  The two arm readings (②a/②b) are compile-level: they prove the unit's text and its
include graph type-check and emit an object under each arm, not that any of it behaves.  C 1024-7950
is **unported**; `extract-abi.py --check`'s 176 `missing …` lines and the 81.5% whole-file drift
are that boundary, not defects.  The two `does not mention` lines are the recorded reading of
`switch`/`goto` (Owed 4).  This ledger claims no runtime evidence, no Android device or emulator
run, and no artifact beyond the two objects named in ② and the empty object of ①.

## Slice 2 (2026-10-08, status 4.106 - the integrator sets the round number): C 996-1022

**What this round is.**  The head of the asset tree: C 999-1002's comment block ("``directory-tree''
is a file in the root of the assets directory … See lib-src/asset-directory-tool"), C 1004-1011's
three file-scope objects, C 1013's comment and **`android_extract_long` (C 1015-1022)** - the
32-bit unaligned reader the whole tree format stands on.  Port: 1,429 -> 1,470 lines.

**The one object this round did *not* write, and why that is the correct reading** (Deviation 14,
and the discharge of Owed 6): C 1005's `static const char *directory_tree;` is a **tentative
definition**, and the object it names is already C's own header's - `android-asset.h` 26 carries
`static const char *directory_tree;` and this unit includes that header at C 51, so in C the two
lines are one object.  The mirror `include/android-asset.rs` 61 carries it (its comment says
"a tentative definition, i.e. a null pointer"), so the unit keeps it out - **measured**: writing it
here is `E0428 the name \`directory_tree\` is defined multiple times`, anchored at that mirror line
and at the unit's.  C's other two objects in the same block are *not* in the header and are the
unit's: `directory_tree_size` (C 1008) and `asset_manager` (C 1011).

**Evidence ① - the arm reading is `exit 0`, before and after (the strongest class in the tree).**
`cargo run -q -p xtask -- probe src/androidvfs.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` → **`exit 0`, 0 errors, 5
warnings** - the same reading HEAD had, and the same one slice 1's ②a/②b recorded.  This is the
reading that makes this file the lane to work in: the unit *and* its include graph type-check
completely, so **anything wrong in new text is reported**, and there is no mirror-error noise to
read around (the survey recorded as status 4.105 put this unit in the top tier while `w32proc`'s arm
set never even reaches type-checking).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error into this slice's
body (`let injected_control: c_int = c"x".as_ptr ();` beside C 1018's local) moves the reading to
**`exit 1` with exactly one error**, `injected_control` named once; reverted and re-run: `exit 0`
again.  Two probes for the round, each ≈1 s, single file.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 179 → 176 rows.**  The three rows
that retired are `missing function android_extract_long` and the two `missing file-scope object`
rows for `directory_tree_size` and `asset_manager`; `directory_tree` was never a row (the mirror
answers it - Owed 6's reading).  The 176 that remain are the C 1024-7950 remainder, and the
whole-file line-count row now reads 81.5%.

**Deviation (this slice's addition).**

14. **`directory_tree`'s single definition is the mirror's - in C's `#else` arm, and only there** (above, and see 15).  The unit records it as a
    comment at C 1004-1005 rather than a definition, because the port's one-spelling rule
    (P-26/`check-mirror-dupes`) resolves the C header's tentative definition to the mirror that
    carries it; the alternative - a unit-local definition - is `E0428` (measured).  The same rule
    will meet C 1024-7950's other shared objects, so it is written down here rather than at each
    site.

**Owed (this slice's additions).**

- Owed 6 (**discharged**): the boundary's first object is the include graph's, and the round that
  crosses the boundary must resolve it with that mirror - done, with the measurement.
- **`tools/check-cond-eval.py src/androidvfs.c` reads `14 conditions, 0 unknown, 5 disagreements
  [NOT VERIFIED]` (exit 1)** - five `STAT_TIMESPEC` rows at C 1980/2162/4346/7323/7820.  All five
  are outside this slice, and the tool reads only the reference file, so the reading is
  pre-existing and not this round's; slice 1's ledger did not record it and this round does, because
  "the ten gates are green" would otherwise be an over-claim for this file.  The clang column says
  `True` for all five where the evaluator says `False`, i.e. the table
  (`docs/baseline/config/darwin-arm64.macros.tsv`) does not carry the configuration header that
  defines `STAT_TIMESPEC`; the mirror round that owns `docs/baseline/config/` owns the row.
- Nothing else: no mirror file is touched, and the round adds no `--check` row.

## Slice 3 (2026-10-08, status 4.107 - the integrator sets the round number): C 1024-1229

**What this round is.**  The asset tree's two readers: `android_scan_directory_tree` (C 1036-1202,
with C 1024-1034's comment) - the function that walks the `directory-tree` file, and whose three C
labels are this slice's control-flow work - and `android_is_directory` (C 1210-1229), which is the
arm pair C 1218/1223 spells.  Port: 1,470 -> 1,738 lines.

**The labels** (PORTING.md 2.2, and `docs/ports/macro-body-notes.tsv`'s new row): C 1106's `again:`
is a *backward* jump, so it is a labelled `loop` whose own end is C 1196's `goto again`; C 1199's
`fail:` is the forward cleanup path, so the body is a labelled block and every `goto fail` is
`break 'fail` while C's label text (C 1200-1201) follows the block, reached by the fall-through as
in C; and C 1176's `false_positive:` is one token's iteration tail, reached by the no-match
fall-through *and* by C 1126's jump, so the matched path sits in a labelled block (C 1121-1173) that
`break 'matched` leaves.  C's `continue` (C 1146) goes to the `for`'s increment, which Rust's
`while` runs at the body's end, so the body is a labelled block and the jump is
`break 'continue_;` - the `src/androidterm.rs` 275-290 shape, cited there.  C's three
`OLD_ANDROID_ASSETS` ternaries are the port's `if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }` (the
constant is `include`'s two-armed `pub const`, slice 1's C 49/52), and C's `#if !OLD_ANDROID_ASSETS`
arms (C 1046/1218) are the `android_api_ge_9` feature's two arms.

**Evidence ① - both arms read `exit 0`, and the second arm is what found this round's repair.**
`cargo run -q -p xtask -- probe src/androidvfs.rs <out> --crate-type=lib -C panic=abort
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` and the same with
`--cfg 'feature="android_api_ge_9"'`: **`exit 0`, 0 errors, 7 warnings each** - the unit and its
include graph type-check under *both* arms of the file's own `#if`.  The first (else-arm) reading
alone was clean too, which is the point: **the `>= 9` arm is what exposed a gap slice 2 had left** -
in that arm C 47-48 includes the NDK's asset-manager headers *instead of* `android-asset.h`, so the
mirror that carries `directory_tree` and `AAssetManager` is not in the graph at all, and the arm
read `E0425 cannot find value directory_tree` at eight sites plus one for the type.  The repair is
C's own structure, one definition per arm: `directory_tree`'s tentative definition and
`pub enum AAssetManager {}` are declared **under `android_api_ge_9` only** (Deviation 15).  This is
the first round in this file whose readings were taken on both arms; slice 2's single-arm reading is
why the gap survived a round.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error into the new
function's local list moves the `>= 9` arm to `exit 1` with the error named once; reverted and
re-run: `exit 0` again.  Four probes for the round (two arms, twice), each about one second.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 176 → 174 rows.**  Two
`missing function` rows retired (`android_scan_directory_tree`, `android_is_directory`) and the one
new spelling row (`false_positive,goto`) was discharged by the note file, so the reading is 174.
The whole-file line-count row now reads 78.1%.

**Deviations (this slice's additions).**

15. **Two names are arm-dependent because C's own include arms are** (Evidence ①).  C 1005's
    `static const char *directory_tree;` and C 47's opaque `struct AAssetManager` exist in the
    `#else` arm through `android-asset.h`'s mirror; in the `>= 9` arm that header is not included
    (C 47-48 reaches the NDK's), so the unit carries both under `#[cfg (feature = "android_api_ge_9")]`
    and only there - the `#else` arm must *not* carry the object, which is slice 2's measured
    `E0428`.  C's text is unconditional because in C both arms see one object; the port's two arms
    need one definition each, which is what C's own preprocessor produces.
16. **The lane now reads both arms of every `#if` it touches**, because this round proved a
    single-arm reading can be silent about a real gap (Evidence ①).  The three `#if`s of this file
    are: the file gate's pair, C 46's `android_api_ge_9`, and C 49/52's `OLD_ANDROID_ASSETS` value
    (which is that feature's other face).

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: the file is now 1,738 of 7,950 C
  lines (21.9%), still above the ten-percent threshold; the integrator's `--write-pending` re-run
  drops the row.
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; the five `STAT_TIMESPEC` rows are C
  1980/2162/4346/7323/7820, all past this slice).

## Slice 4 (2026-10-08, status 4.108 - the integrator sets the round number): C 1230-1287

**What this round is.**  `android_init_assets` (C 1235-1287, with C 1231-1233's comment) - the
function that binds the Java asset manager to the native side, opens `directory-tree` itself and
checks its magic.  Port: 1,738 -> 1,839 lines.

**The round's arm work, written before the probe this time** (the slice-3 lesson): C's two arms get
this function's five `<android/asset_manager.h>` names from two different places - the `#else` arm
from C 51's `android-asset.h` mirror, whose emulation *defines* `AAssetManager_fromJava`,
`AAssetManager_open`, `AAsset_getBuffer`, `AAsset_getLength` and `AASSET_MODE_BUFFER`
(`include/android-asset.rs` 136/138/257/565/587), and the `>= 9` arm directly from the NDK's header,
which has no mirror - so the unit declares all five under `#[cfg (feature = "android_api_ge_9")]`
(Deviation 17).  C 1262-1278's `#if !OLD_ANDROID_ASSETS` arm pair is the same feature's two arms,
each carrying its own magic check (`"EMACS"`/5 bytes and `"EMACS____"`/9 bytes) with C's
`__android_log_print (ANDROID_LOG_FATAL, …)` and `emacs_abort ()`.

**Evidence ① - both arms read `exit 0`.**  The probe with the else arm and with
`--cfg 'feature="android_api_ge_9"'`: **`exit 0`, 0 errors, 4 warnings each**.  Three real defects
came out of the two first drafts, and the arms found them in turn: a unit-local
`pub type off_t = i64;` was `E0428 the name \`off_t\` is defined multiple times` in the `>= 9` arm
(C 32's `sys_types.in.rs` edge already carries it - so the declaration is *not* written, and the
comment says why); and C 1285's `((*env)->ExceptionCheck (env))` used as a condition is the port's
`… != 0`, because the JNI entry returns `jboolean` (a `u8`) - the `E0308 expected bool, found u8`
that the else arm reported first.  Both fixes are the same class this ledger already records:
C's implicit conversions written out.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error into the new
function's local list moves the `>= 9` arm to `exit 1` with the error named once; reverted and
re-run: `exit 0`.  Three probes for the round, each about one second.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 174 → 173 rows**, the retired row
being `missing function android_init_assets`, **and no new spelling row**: the body's tokens match
C's (the `"directory-tree"`/`"EMACS"`/`"EMACS____"` literals, `AASSET_MODE_BUFFER`, the two JNI
calls and both magic arms are all compared).  The whole-file line-count row now reads 76.9%.

**Deviation (this slice's addition).**

17. **Five names are arm-dependent for the same reason two were in slice 3** (Evidence ①).  C 47-48
    is the `>= 9` arm's include of the NDK's asset-manager headers and C 51 is the `#else` arm's
    include of `android-asset.h`; a name only the `#else` mirror carries must therefore be declared
    by the unit for the other arm, and a name only the NDK header carries has to be declared by the
    unit anyway (P-88 ① - no mirror exists for an NDK header).  The rule this lane follows from here
    on: **for every name this file takes from `<android/asset_manager.h>`, ask which arm carries
    it**, because the answer decides whether the unit declares it and under which `#[cfg]`.

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: 1,839 of 7,950 C lines (23.1%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5 disagreements
  [NOT VERIFIED]` (slice 2's owed item; the five rows are all past this slice).
