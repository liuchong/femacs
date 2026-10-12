# emacs/src/androidvfs.c (7,950 lines) — slice 1: C 1-995

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2647-2648 adds `androidvfs.o` to `ANDROID_OBJ` only in the really-Android branch
> (`XCONFIGURE = android`, 2654), while 2597's list (`androidmenu.o`) is in both and 2605-2606
> defines `ANDROID_STUBIFY` in the other one — so this file's build condition is C's
> `HAVE_ANDROID && !defined ANDROID_STUBIFY`, `docs/baseline/frozen-build-sources.txt` lists no
> `androidvfs.o` for this target, and `extract-abi.py`'s checklist for the file is empty (P-86
> ⑩: its checkbox sections cannot be coverage evidence).  What follows is P-86 ②'s substitute:
> the **C line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/androidvfs.rs` (**8,730 lines**, md5 `008cbad12db6977b0658cedb6138d1e1`), carrying
> **C 1-6852** (slice 19 closed every deferral this lane had accumulated) (C 2520-2537's `content_vfs_ops`, C 2549-2639's
> `android_content_name`, C 2887-2906's `android_content_initial`, C 3688-3705's `saf_root_vfs_ops`,
> C 3748-3848's `android_saf_root_name` and C 4209-4229's `android_saf_root_initial` - one
> dependency chain that the round reaching C 5773 closes, Deviation 23/27 - plus, this round,
> C 4752-4769's `saf_tree_vfs_ops`, C 4897-5112's `android_saf_tree_name` - which also needs the two
> tentative vectors C 4771-4777 declares, `saf_file_vfs_ops`/`saf_new_vfs_ops`, whose initialisers
> are in the pages after C 6200 - C 5761-5846's `android_saf_tree_from_name` and C 6183's alias
> `#define android_saf_new_vnode android_saf_tree_vnode`, whose `type` alias lands with that family) -
> P-86 ④'s first seventeen slices of this file - with both arms of every `#if` carried.  The unit holds two items out
> of C's order (both at the file's foot: `afs_vfs_ops`, Deviation 21, and `authority_vfs_ops`,
> Deviation 26).  Slice 8 carries the one item in the unit that sits out of C's order
> (`afs_vfs_ops`, C 1682-1699, at the file's foot; Deviation 21).
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
| 1289-1355 | the asset-to-fd section's opening (C 1291) and `asharedmemory_create` (C 1295), then `android_hack_asset_fd_fallback` (C 1301-1355; C 1352's `fail:` as the port's labelled block) | file gate (slice 5); C 1356 is a blank line |
| 1357-1611 | the ashmem pair - `android_detect_ashmem` (C 1363-1465, with C 1357-1361's comment) and `android_hack_asset_fd` (C 1470-1611, with C 1467-1468's comment) | file gate (slice 6); C 1612 is a blank line |
| 1613-1680 | the asset-VFS banner (C 1615-1617), the three structs (`android_afs_vnode` C 1619-1629, `android_afs_vdir` C 1631-1650, `android_afs_open_fd` C 1652-1662) and C's twelve forward declarations (C 1664-1680, which Rust needs no text for) | file gate (slice 7) |
| 1681-1700 | `afs_vfs_ops` (C 1682-1699), the thirteen-entry operation vector - **landed at this file's foot, out of C's order** | file gate (slice 8); Deviation 21 (the P-91 clique) |
| 1701-1706 | the two file-scope objects `all_afs_vdirs` (C 1702) and `afs_file_descriptors` (C 1706) | file gate (slice 7) |
| 1707-1844 | `android_afs_name` (C 1708-1820, with C 1708's declaration comment) and `android_afs_initial` (C 1826-1843, with C 1822-1824's comment) | file gate (slice 8) |
| 1845-1870 | `android_close_on_exec` (C 1848-1870, with C 1845-1846's comment) | file gate (slice 7) |
| 1871-2466 | the rest of the asset family: `android_afs_open` (C 1872-2003) through `android_afs_get_directory_name` (C 2454-2466), with C 2449-2452's comment | file gate (slice 8); C 2467-2469 are blank |
| 2470-2519 | `struct android_content_vdir` (C 2470-2484), C 2486-2488's three forward declarations, C 2490-2500's comment on the content meta-interface and C 2502-2518's twelve more (Rust needs no text for any forward declaration) | file gate (slice 9) |
| 2520-2537 | `content_vfs_ops` (C 2522-2537) | file gate (slice 19: the clique Deviation 23 named is closed here) |
| 2539-2548 | `content_directory_contents` (C 2541-2544) and `all_content_vdirs` (C 2547) | file gate (slice 9) |
| 2549-2639 | `android_content_name` (C 2549-2639, with its function-scope `content_vnodes[]` and C's `goto parent_vnode` as a labelled block) | file gate (slice 19) |
| 2641-2885 | the fifteen content functions (`android_content_open` … `android_content_get_directory_name`) with their comments | file gate (slice 9) |
| 2887-2906 | `android_content_initial` (C 2887-2906) | file gate (slice 19) |
| 2908-2946 | C 2908-2916/2942-2946's `#pragma` pair (a record), C 2918's comment and `displayNameHash` (C 2920-2940, the JNI entry point) | file gate (slice 9) |
| 2948-3034 | C 2948-2954's comment and `android_get_content_name` (C 2956-3034, C's `goto no_checksum` as the port's labelled block) | file gate (slice 10) |
| 3036-3066 | C 3036-3039's comment and `android_check_content_access` (C 3041-3066) | file gate (slice 10) |
| 3068-3070 | C 3069's form feed and the comment `/* Functions shared by authority and SAF nodes.  */` | file gate (slice 10) |
| 3072-3164 | C 3072-3093's doc comment and `android_saf_exception_check` (C 3095-3164, variadic) | file gate (slice 10) |
| 3166-3195 | C 3166-3169's comment and `android_saf_check_nonnull` (C 3171-3195, variadic) | file gate (slice 10) |
| 3197-3234 | C 3199-3206's comment, `struct android_authority_vnode` (C 3208-3216) and C 3218-3234's twelve forward declarations (no Rust text) | file gate (slice 11) |
| 3236-3253 | `authority_vfs_ops` (C 3238-3253), **landed at this file's foot, out of C's order** | file gate (slice 11); Deviation 26 (the P-91 clique) |
| 3255-3335 | `android_authority_name` (C 3255-3335, C's `goto no_entry`/`goto error` as the port's labelled block plus the written-out `error:` path) | file gate (slice 11) |
| 3337-3395 | `android_authority_open` (C 3337-3395) | file gate (slice 11) |
| 3397-3408 | `android_authority_close` (C 3397-3408) | file gate (slice 11) |
| 3410-3561 | the nine small functions (`android_authority_unlink` … `android_authority_opendir`) | file gate (slice 11) |
| 3563-3613 | `android_authority_initial` (C 3563-3587) and `android_authority_initial_name` (C 3589-3613) with their comments | file gate (slice 11) |
| 3617-3686 | C 3617-3636's comment, `struct android_saf_root_vnode` (C 3638-3646), `struct android_saf_root_vdir` (C 3648-3668) and C 3670-3686's twelve forward declarations (no Rust text) | file gate (slice 12) |
| 3688-3705 | `saf_root_vfs_ops` (C 3690-3705) | file gate (slice 19) |
| 3707-3746 | `all_saf_root_vdirs` (C 3708), C 3710-3712's forward declaration of `android_saf_tree_from_name` (no Rust text; Deviation 27) and `android_saf_valid_authority_p` (C 3714-3746) | file gate (slice 12) |
| 3748-3848 | `android_saf_root_name` (C 3748-3848, C's `goto parent_vnode` as a labelled block) | file gate (slice 19) |
| 3850-4207 | the fifteen SAF root members (`android_saf_root_open` … `android_saf_root_opendir`) with their comments | file gate (slice 12) |
| 4209-4229 | `android_saf_root_initial` (C 4209-4229) | file gate (slice 19) |
| 4231-4246 | C 4231-4232's comment and `android_saf_root_get_directory` (C 4234-4246) | file gate (slice 12) |
| 4248-4254 | C 4250's comment, C 4252-4253's comment and `inside_saf_critical_section` (C 4254) | file gate (slice 13) |
| 4256-4445 | C 4256-4263's doc comment and `android_saf_stat` (C 4265-4358, with C 4346-4354's `#ifdef STAT_TIMESPEC` pair), then C 4360-4369's doc comment and `android_saf_access` (C 4371-4445, C's `switch` as the port's `match`) | file gate (slice 13) |
| 4447-4497 | C 4447-4454's doc comment and `android_saf_delete_document` (C 4456-4497) | file gate (slice 13) |
| 4499-4501 | C's `android_document_id_from_name` forward declaration under `/* Declared further below.  */` (no Rust text) | file gate (slice 13) |
| 4503-4572 | C 4503-4513's doc comment and `android_saf_rename_document` (C 4515-4572) | file gate (slice 13) |
| 4574-4677 | `android_saf_move_document` (C 4584-4677, C's `switch (rc)` as the port's `match`) with C 4574-4582's doc comment | file gate (slice 14) |
| 4679-4732 | C 4681-4685's comment, `struct android_saf_tree_vnode` (C 4687-4712) and `struct android_saf_tree_vdir` (C 4714-4732) | file gate (slice 14) |
| 4734-4750 | C 4734-4750's twelve forward declarations (no Rust text) | file gate (slice 14) |
| 4752-4769 | `saf_tree_vfs_ops` (C 4754-4769) | file gate (slice 19) |
| 4771-4777 | C 4771-4777's two tentative vector declarations, `saf_file_vfs_ops` and `saf_new_vfs_ops`, under their own comments (both are defined in the pages after this one) | file gate (slice 14); Deviation 29 |
| 4779-4780 | `all_saf_tree_vdirs` (C 4780) with C 4779's comment | file gate (slice 14) |
| 4782-4895 | C 4782-4803's doc comment and `android_document_id_from_name` (C 4805-4895) | file gate (slice 14) |
| 4897-5112 | `android_saf_tree_name` (C 4897-5112, the largest item of the deferred set, C's `goto parent_vnode` as a labelled block) | file gate (slice 19) |
| 5114-5173 | `android_saf_tree_open` (C 5114-5122), `close` (C 5124-5139), `unlink` (C 5141-5146), `symlink` (C 5148-5153) and `rmdir` (C 5155-5173) | file gate (slice 15) |
| 5175-5403 | `android_saf_tree_rename` (C 5176-5403, C's five `goto error` sites and the `error:` label as the port's labelled block inside the second case) | file gate (slice 15) |
| 5405-5471 | `android_saf_tree_stat` (C 5405-5415), `access` (C 5417-5434), `mkdir` (C 5436-5443), `chmod` (C 5445-5461) and `readlink` (C 5463-5471) | file gate (slice 15) |
| 5473-5529 | C 5473-5477's comment and `android_saf_tree_opendir_1` (C 5479-5529) | file gate (slice 15) |
| 5531-5626 | `android_saf_tree_readdir` (C 5531-5626, with C 5594's `coding.mode |=` as the mirror's bitfield write) | file gate (slice 16) |
| 5628-5662 | `android_saf_tree_closedir` (C 5628-5662) | file gate (slice 16) |
| 5664-5685 | `android_saf_tree_dirfd` (C 5664-5685) | file gate (slice 16) |
| 5686-5759 | `android_saf_tree_opendir` (C 5686-5759) | file gate (slice 16) |
| 5761-5846 | `android_saf_tree_from_name` (C 5761-5846) | file gate (slice 19) |
| 5848-5863 | C 5848-5849's comment and `android_saf_tree_get_directory` (C 5851-5863) | file gate (slice 16) |
| 5865-5876 | C 5867-5869's comment, C 5871-5874's `#define android_saf_file_vnode android_saf_tree_vnode` alias (the port's `pub type`) and C 5876's comment | file gate (slice 17) |
| 5878-5919 | `struct android_parcel_fd` (C 5878-5892), C 5894-5900's five forward declarations (no Rust text) and `saf_file_vfs_ops` (C 5902-5919, the tentative vector's initialiser) | file gate (slice 17) |
| 5921-5922 | `open_parcel_fds` (C 5922) with C 5921's comment | file gate (slice 17) |
| 5924-5950 | `android_saf_file_name` (C 5924-5950) | file gate (slice 17) |
| 5952-6106 | `android_saf_file_open` (C 5952-6106) | file gate (slice 17) |
| 6108-6131 | `android_saf_file_unlink` (C 6108-6117), `_rmdir` (C 6119-6124) and `_opendir` (C 6126-6131) | file gate (slice 17) |
| 6133-6170 | C 6133-6138's comment and `android_close_parcel_fd` (C 6140-6170) | file gate (slice 17) |
| 6172-6200 | C 6172-6181's form feed and comment on the SAF ``new'' vnodes, C 6183's `#define android_saf_new_vnode android_saf_tree_vnode` (the port's `type` alias) and C 6185-6200's twelve forward declarations (no Rust text) | file gate (slice 18) |
| 6202-6219 | `saf_new_vfs_ops` (C 6204-6219) | file gate (slice 19) |
| 6221-6247 | `android_saf_new_name` (C 6221-6247) | file gate (slice 18) |
| 6249-6339 | `android_saf_new_open` (C 6249-6339, which transforms the vnode into a file vnode and calls through its own `open`) | file gate (slice 18) |
| 6341-6388 | `android_saf_new_unlink` (C 6341-6346), `_symlink` (C 6348-6353), `_rmdir` (C 6355-6360), `_rename` (C 6362-6369), `_stat` (C 6371-6377) and `_access` (C 6379-6388) | file gate (slice 18) |
| 6390-6477 | `android_saf_new_mkdir` (C 6390-6477) | file gate (slice 19) |
| 6479-6500 | `android_saf_new_chmod` (C 6479-6485), `_readlink` (C 6487-6493) and `_opendir` (C 6495-6500) | file gate (slice 18) |
| 6502-6505 | C 6503's form feed and the comment on SAF/Emacs synchronization | file gate (slice 18) |
| 6507-6508 | C 6507's comment and `saf_completion_sem` (C 6508, with the `<semaphore.h>` declarations at their use site) | file gate (slice 20) |
| 6510-6516 | C's `#ifdef __clang__` / `#pragma … push/ignored` / `#else` / `#pragma GCC …` / `#endif` pair (a record) | file gate (slice 20) |
| 6518-6571 | the four JNI entry points: `safSyncAndReadInput` (C 6518-6539), `safSync` (C 6541-6548), `safPostRequest` (C 6550-6556) and `ftruncate` (C 6558-6571, whose pasted name collides with the mirror's system entry, so its definition stands in a module - Deviation 33) | file gate (slice 20) |
| 6573-6577 | C's `#pragma` pop (a record) | file gate (slice 20) |
| 6579-6585 | C 6580's form feed and the comment on the root vnode | file gate (slice 20) |
| 6587-6597 | `struct android_root_vdir` (C 6587-6597) | file gate (slice 20) |
| 6599-6604 | `root_fd` (C 6601) and `root_fd_references` (C 6604) with their comments | file gate (slice 20) |
| 6606-6608 | C's two forward declarations of `android_root_name`/`android_root_opendir` (no Rust text) | file gate (slice 20) |
| 6610-6628 | `root_vfs_ops` (C 6613-6628) with C 6610-6611's comment | file gate (slice 20) |
| 6630-6637 | `special_vnodes` (C 6632-6637, whose second entry carries the `LISPSYM_INITIALLY (Qandroid_jni)` constant expression) | file gate (slice 20) |
| 6639-6659 | C 6639-6647's comment and `android_vfs_convert_name` (C 6649-6659) | file gate (slice 20) |
| 6661-6766 | `android_root_name` (C 6661-6766, with the two `USE_SAFE_ALLOCA` blocks and the `special_vnodes` loop) | file gate (slice 20) |
| 6768-6789 | `android_root_readdir` (C 6768-6789) | file gate (slice 20) |
| 6791-6811 | `android_root_closedir` (C 6791-6811, with C's post-decrement pair) | file gate (slice 20) |
| 6813-6818 | `android_root_dirfd` (C 6813-6818) | file gate (slice 20) |
| 6820-6852 | `android_root_opendir` (C 6820-6852) | file gate (slice 20) |
| 6854-7950 | **the remainder** - C 6854's form feed and the file-system lookup comment through C 7950: `android_name_file`, `android_vfs_init`, the android_* wrappers, `android_vfs_ops` and `syms_of_androidvfs` | not in this slice: recorded here so the table covers the reference, to be replaced by the next lane's rows |

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
   **stale** (**8,730** C lines carried = 109.8% of 7,950, above the ten-percent threshold; slice 1's
   original reading was C 995 = 12.5%); the integrator's `--write-pending` re-run drops it.
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
`opendir`/`readdir`/`fstatat`/`rename`/`symlink`, no asset was read, no `open`/`unlink`/`ftruncate`
/`mmap`/`munmap` was called, and no vnode was ever created, named, opened or closed.  The arm
readings (slice 5's Evidence ①) are compile-level: they prove the unit's text and its include graph
type-check and emit an object under each arm, not that any of it behaves.  C 6854-7950 is
**unported**: `extract-abi.py --check` reads **37** rows across that boundary (31 `missing …`, five
`invented exported symbol` rows - the `NATIVE_NAME` paste class that `docs/ports/android.md` 432/462
records for `android.rs`, now including this file's four entries - and the whole-file line-count
row, which is back at **9.8%** because the port now carries more lines than the reference:
8,730 against 7,950), and those rows are the boundary, not defects.

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

## Slice 5 (2026-10-08, status 4.109 - the integrator sets the round number): C 1289-1355

**What this round is.**  The asset-to-file-descriptor section's head - C 1291's one-line banner, C
1293-1294's comment and C 1295's `asharedmemory_create` pointer - and
`android_hack_asset_fd_fallback` (C 1301-1355), the unlinked-temporary-file fallback for the old
kernels where an ashmem descriptor cannot be read back; C 1352's `fail:` label becomes the port's
labelled block.  C 1297-1299's comment is carried.  Port: **1,839 -> 1,966 lines**.

**The round's arm work, written before the probe** (Deviation 16's rule, Deviation 17's applied once
more): C 1340's `AAsset_read` is the seventh name of that header family, and the two arms carry it
in two different places - the `#else` arm through `include/android-asset.rs` 613's emulation, the
`>= 9` arm through the NDK's `<android/asset_manager.h>`, which has no mirror (P-88 ①) - so the
unit declares it **under `#[cfg (feature = "android_api_ge_9")]` alone**, and both arms were probed
with that declaration in place.  The five remaining names have no arm at all and are declared once:
`mmap`/`munmap` (the `src/pdumper.rs` 104-112 shape), `PROT_WRITE` 0x2 (the Linux value,
`src/gmalloc.rs` 573), `MAP_SHARED` 0x01, `MAP_FAILED` (as `usize::MAX as *mut c_void`, the
`(void *) -1` of `src/pdumper.rs` 186), `android_cache_dir` (C's `android.h` 49 - `include/
android.rs` 227 carries it, but that mirror is not in this unit's include graph, P-72 rule 4), and
`ANDROID_LOG_ERROR` (**6**, measured from this machine's NDK 25.2.9519653 `android/log.h` 73-90;
the `WARN` 5 / `FATAL` 7 neighbours are already declared above).

**Evidence ① - both arms read `exit 0`.**  The probe without a second flag: **`exit 0`, 0 errors,
7 warnings**; with `--cfg 'feature="android_api_ge_9"'`: **`exit 0`, 0 errors, 11 warnings** -
diffed, the four extra ones are `extern block uses type \`AAsset\`, which is not FFI-safe`, one per
`AAsset*` declaration the `>= 9` arm carries (this round's `AAsset_read` is the fourth).  Both readings are the unit's text
plus its include graph type-checking - the strongest class this tree has for a file no target
compiles.  Two real defects came out of the first draft, one per arm: ⑴ the labelled block must
*enclose* C 1327/1330/1337/1346's break sites (C's label text follows the block; a first draft that
put the block first and the label after it read `E0425 cannot find value \`fd\`` at the `break`
sites); ⑵ C 1320's `open` is variadic and C promotes its `mode_t` argument, so the port writes
`(S_IRUSR | S_IWUSR) as c_int` - without the cast the call reads `E0617 can't pass `u32` to
variadic C function` (C androidvfs.c 1190's own `open` call is spelled the same way).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 1304's
local declaration moves the `>= 9` arm to **`exit 1`, `aborting due to 1 previous error`**, with
`injected_control` named **once**; reverted, the same arm reads `exit 0` again and the file's md5
returns to the header's `8f0cf10d34c562e574a0c0eb127a2a13`.  Three probes this round, each about a
second.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 173 -> 172 rows**, the retired row
being `missing function android_hack_asset_fd_fallback`.  **No new spelling row appeared**: the
round's own `goto` row (`androidvfs.c 1327/1330/1337/1346`'s four `goto fail` and C 1352's `fail:`
against the port's labelled block) is registered in `docs/ports/macro-body-notes.tsv` in the same
commit - every other token of the body (`AAsset_getLength`/`AAsset_read`, `snprintf`'s format and
arguments, `open`'s three flags, `unlink`/`ftruncate`/`mmap`/`munmap`/`close`, `errno`/`ENOENT`,
`MAP_FAILED`/`PROT_WRITE`/`MAP_SHARED`, both `__android_log_print` calls) is compared and matches.
The whole-file line-count row now reads **75.3%** (C 7,950 against 1,966 Rust lines).

**Evidence ④ - the ten gates, and one of them was red at HEAD.**  Nine read as they did last round
(`check-own-voice` 0, `check-mirror-dupes` 17/17/0, `check-port-lines` 0/0/0/0, `check-ledger-ticks`
238/0, `check-carried` 11/11/4/0, `check-verbatim` 6 trees/4,604 files/0, `check-verbatim-
declarations` 0, `check-dead-file-coverage` 145/0, `check-citations` 676/0/0/2,556/36/0), but
`check-declarations` read **20 declaration sites (2 forbidden, 18 allowed literals), 4 failures** -
all four in **another unit** (`femacs/src/haikuterm.rs`'s C 4536/4542/4547 text and the matching
ledger line), committed state this slice does not touch.  The integrator fixed that as round 4.110;
that reading is recorded there, and this round's own text contributes none of the four.

**Deviation (this slice's addition).**

18. **C's `fail:` label is a labelled *block* here, and the block encloses the jump sites** (Evidence
    ①).  `c`-level `goto fail` sites at C 1327/1330/1337/1346 all jump *forward* to C 1352's label,
    which sits after the function's main body; the port spells that as `'fail: { ... }` with
    `break 'fail;` at the four sites and the label's own statements after the block - the shape
    PORTING.md 2.2 records and the `android_unix_name` / `android_scan_directory_tree` rows already
    carry (their note rows are the same class).  Written the other way round (block first, C's
    label's text after it) the `break` sites lose their target and read `E0425`.

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **1,966** of 7,950 C lines (24.7%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; the five rows are all past this slice).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  C 1340's `AAsset_read` was never
called, no temporary file was created or unlinked, nothing was mapped, and the section's
`asharedmemory_create` pointer is still `None` (its loader is C 1363's `android_detect_ashmem` and
later).  What the round's evidence *is*: both arms' `exit 0`, one control proving the harness reads
this text, `--check` 173 -> 172 with no new row, and the nine gates plus the haikuterm-attributed
red one named above.  **This file is not finished**: C 1357-7950 remains (the next item is C 1363's
`android_detect_ashmem`, C 1357-1361's comment).

## Slice 6 (2026-10-08, status 4.111 - the integrator sets the round number): C 1357-1611

**What this round is.**  The ashmem pair: `android_detect_ashmem` (C 1363-1465, with C 1357-1361's
comment) - the `/dev/ashmem` probe that decides whether a shared-memory descriptor can be read back,
with C's two API-level arms (`android_get_current_api_level () <= 28` opens `/dev/ashmem` and sets
its name and size with two `ioctl`s; the `else` arm loads `ASharedMemory_create` with `dlsym`) - and
`android_hack_asset_fd` (C 1470-1611, with C 1467-1468's comment), the asset-to-descriptor function
that caches that decision in two function-scope `static bool`s and then runs the same two arms again.
Port: **1,966 -> 2,281 lines**.

**The round's use-site declarations, written before the probe** (P-88 ①, Deviation 19): five names no
mirror this unit includes carries - `<dlfcn.h>`'s `dlsym` and its `RTLD_DEFAULT` (measured 0 in
bionic's header, the `femacs/exec/trace.rs` 503-513 shape), `<sys/ioctl.h>`'s variadic `ioctl` (the
`femacs/src/inotify.rs` 222 spelling), `<linux/ashmem.h>`'s two constants (**measured** with this
machine's NDK 29.0.14206865 through its clang: `ASHMEM_SET_NAME` 0x41007701 and `ASHMEM_SET_SIZE`
0x40087703 - `__ASHMEMIOC` is 0x77, `ASHMEM_NAME_LEN` 256, and `_IOW` packs
`1U << 30 | sizeof << 16 | type << 8 | nr`, so neither is the `'a'`-based value a reader would guess)
- and android.h 148's `android_get_current_api_level`.

**Evidence ① - both arms read `exit 0`.**  The probe without a second flag and with
`--cfg 'feature="android_api_ge_9"'`: **`exit 0`, 0 errors** - 7 warnings in the else arm and 11 in
the `>= 9` arm, the same two counts slice 5's tail read, so this round's text adds no warning.  Six real defects came out of the first
two drafts, all the class this ledger already records - C's implicit conversions written out:

- C's adjacent string literals C 1455-1458 are `concat!` with an explicit `"\0"` and a
  `as *const c_char` cast (the `femacs/src/comp.rs` 627/640 spelling); the first draft's
  `concat! (...).as_ptr ()` passed a `*const u8` where the variadic call wants `*const i8`, and
  dropping `"\0"` would have left the C string unterminated;
- C 1531's `if (!size)` is `if size == 0` (a `size_t` is not a `bool`);
- C 1476's `unsigned char *mem` receiving `mmap`'s `void *` is four written-out casts - two
  `… as *mut c_uchar` on the assignments (the `femacs/src/w32.rs` 3921 shape) and two
  `mem as *mut c_void == MAP_FAILED` comparisons, plus the two `AAsset_read`/`munmap` argument
  casts, which were written that way in the first draft.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 1366's
local declaration moves the else arm to **`exit 1`, `aborting due to 1 previous error`**, with
`injected_control` named **once**; reverted, the same arm reads `exit 0` again and the file's md5
returns to the header's `81447da3a382109c830a74d26e5af290`.  Three probes this round, each about a
second.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 172 -> 170 rows**, the two retired
rows being `missing function android_detect_ashmem` and `missing function android_hack_asset_fd`.
**No new spelling row appeared**: every token both bodies use (`memcpy`/`memcmp`/`read`/`mmap`/
`munmap`/`ioctl`/`open`/`close`/`dlsym`/`strerror`/`errno`/`emacs_abort`, the eight
`__android_log_print` calls with their three measured priorities (`WARN` 5, `ERROR` 6, `FATAL` 7), `ASHMEM_SET_NAME`/
`ASHMEM_SET_SIZE`/`RTLD_DEFAULT`, both `android_get_current_api_level () <= 28` tests, `sizeof
test_buffer`'s ten and the `"abcdefghi"` pair) is compared and matches.  The whole-file line-count
row now reads **71.3%** (C 7,950 against 2,281 Rust lines).

**Deviation (this slice's addition).**

19. **C's `android_get_current_api_level` is a macro, and the port calls the function.**  C 1377 and
    C 1499 read `android_get_current_api_level ()`, which android.h 152-159 rewrites to
    `({ int value; value = (android_get_current_api_level) (); eassume (value >= __ANDROID_API__);
    value; })` - a cached read plus a compile-time hint whose only content is the NDK's own
    `__ANDROID_API__` floor.  This unit declares the *function* (android.h 148's prototype, under
    this unit's gate) and writes C's call as C writes it, because the tree already decided this
    class the same way in a unit that *does* carry the mirror: `femacs/src/androidselect.rs` 585/613
    keeps C's `android_get_current_api_level () <= 11`/`<= 30` tests and records the invariant in
    its ledger (its 23/581 lines).  The macro's two other halves are recorded rather than dropped:
    `__ANDROID_API__` is the NDK's `__ANDROID_MIN_SDK_VERSION__`, **measured 24** with this machine's
    NDK (`femacs/src/androidmenu.rs` 68-73 is the measured row this one repeats), and the `eassume`
    is `femacs/src/include/lisp.rs` 407's macro, which the `#[cfg (enable_checking)]` arms alone
    compile - so a ported call site cannot observe it either way.  The port therefore reads the same
    value C reads; what it does not carry is the *hint*.

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **2,281** of 7,950 C lines (28.7%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; the five rows are all past this slice).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  `/dev/ashmem` was never opened, no
`ioctl` ran, no `ASharedMemory_create` was resolved (the pointer is still `None` in every arm this
machine can build), nothing was mapped, and the two `static bool`s are still `false`/`false`.
What the round's evidence *is*: both arms' `exit 0`, one control proving the harness reads this
text, `--check` 172 -> 170 with no new row, the measured NDK constants, and the ten gates.
**This file is not finished**: C 1613-7950 remains (the next item is C 1615's asset-VFS banner and
the forward declarations under it).

## Slice 7 (2026-10-08, status 4.112 - the integrator sets the round number): C 1613-1680, C 1701-1706 and C 1845-1870

**What this round is.**  The asset-VFS banner (C 1615-1617), its three structs - `android_afs_vnode`
(C 1619-1629), `android_afs_vdir` (C 1631-1650) and `android_afs_open_fd` (C 1652-1662) - C's twelve
forward declarations (C 1664-1680, which Rust needs no text for), the two file-scope objects
`all_afs_vdirs` (C 1702) and `afs_file_descriptors` (C 1706), and `android_close_on_exec`
(C 1848-1870, with C 1845-1846's comment).  Port: **2,281 -> 2,439 lines**.

**What this round defers, and why it is not a choice (Deviation 20).**  `afs_vfs_ops` (C 1682-1699)
names twelve functions C defines at C 1873-2900; Rust has no forward declaration (PORTING.md 10.10,
the P-91 class), while two of those twelve - `android_afs_name` (C 1740) and `android_afs_initial`
(C 1834) - name the object in return.  The three are one dependency clique, so the object's text has
to move to the foot of the round that reaches C 1873 (the haikuterm slice-22 out-of-order device,
its Deviation 34), and this round cannot land any of them: measured, a first draft that carried the
object read `E0425 cannot find value \`android_afs_open\`` (and eleven more, one per entry) in
**both** arms.  The range table carries the two deferred rows, and the unit records the deferral
where C's text would be.

**Evidence ① - both arms read `exit 0`.**  The probe with the else arm and with
`--cfg 'feature="android_api_ge_9"'`: **`exit 0`, 0 errors**, 7 warnings in the else arm and 11 in
the `>= 9` arm - the same two counts the previous two rounds read, so this round's text (which
includes one new private function) adds no warning.  One real defect came out of the first draft:
`android_afs_open_fd` embeds the *mirror's* `stat`, which models the frozen target - the NDK's own
`struct stat` is 128 bytes on `aarch64-linux-android24` (measured with this machine's clang), while
`lib/include/sys_stat.in.rs` 108 asserts 144 and the port carries only that one type, so the
assertion this round writes is **160** (16 + 144) and the 128 is recorded in the note beside it.
The other assertions are the measurements the NDK's clang gives directly: `android_afs_vnode` 32
(vnode 0, name_length 16, name 24), `android_afs_vdir` 64 (vdir 0, next 24, asset_dir 32,
asset_limit 40, asset_file 48, fd 56), `android_afs_open_fd` (next 0, fd 8, statb 16).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 1853's
`fcntl` call moves the else arm to **`exit 1`, `aborting due to 1 previous error`**, with
`injected_control` named **once**; reverted, the same arm reads `exit 0` again and the file's md5
returns to the header's `2887d43a6100b9b58e3b2cc39406f`.  Three probes this round, each about a
second.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 170 -> 167 rows, and the row set was
*diffed*, not eyeballed**: the three retired rows are `missing function android_close_on_exec`,
`missing file-scope object afs_file_descriptors` and `missing file-scope object all_afs_vdirs`, **no
row appears**, and the whole-file line-count row moves 71.3% -> **69.3%** (C 7,950 against 2,439 Rust
lines).  The three structs are rows in neither reading (this file's checklist reports functions,
subrs, macros, file-scope objects and the line count).

**Deviation (this slice's addition).**

20. **One dependency clique is deferred as a whole** (Evidence ①, and the note where the object's
    text would be): `afs_vfs_ops` + `android_afs_name` + `android_afs_initial` cannot be split
    across rounds, because Rust has no forward declaration and the references run both ways.  The
    rule this lane follows from here on: **before writing an object whose initialiser names
    functions C defines later, ask whether those functions name the object back** - if they do, the
    object's text moves to the foot of the round that lands the last of them, and the range table
    carries a `deferred` row naming the reason.  (The alternative the tree already rejects is an
    `Option` full of `None`s: a stub.)

**Owed (this slice's additions).**

- The two deferred ranges (C 1682-1699, C 1708-1843) are the next round's first items.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **2,439** of 7,950 C lines (30.7%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; the five rows are all past this slice).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No vnode of this family was ever
created, named, opened or closed, and `all_afs_vdirs`/`afs_file_descriptors` are still null pointers.
What the round's evidence *is*: both arms' `exit 0`, one control proving the harness reads this text,
`--check` 170 -> 167 with the retiring rows named, the measured layouts, and the ten gates.  **This
file is not finished**: C 1871-7950 remains (the next item is C 1873's `android_afs_open`), and that
round opens with the two deferred ranges.

## Slice 8 (2026-10-08, status 4.113 - the integrator sets the round number): C 1708-2466 plus C 1682-1699 at the foot (the asset-VFS family closes)

**What this round is.**  The whole asset-VFS family, i.e. the eighteen items C 1708-2466 -
`android_afs_name`, `android_afs_initial`, `android_afs_open`, `android_afs_close`,
`android_afs_unlink`, `android_afs_symlink`, `android_afs_rmdir`, `android_afs_rename`,
`android_afs_stat`, `android_afs_access`, `android_afs_mkdir`, `android_afs_chmod`,
`android_afs_readlink`, `android_afs_readdir`, `android_afs_closedir`, `android_afs_dirfd`,
`android_afs_opendir`, `android_afs_get_directory_name` - with `afs_vfs_ops` (C 1682-1699) at the
file's foot.  Port: **2,439 -> 3,364 lines** (925 lines this round).

**Evidence ① - both arms read `exit 0`.**  `exit 0`, 0 errors; 7 warnings in the else arm and 12 in
the `>= 9` arm (one more than the previous round, and diffed: the extra one is the FFI-safety note
for the new `AAsset_close` declaration, so there are now five - one per `AAsset*` name that arm
carries).  Five real defects came out of the first drafts: the `for (next = &all_afs_vdirs; (tem =
*next);)` loop's step is `&raw mut (**next).next` (a `*mut *mut T` does not auto-deref for a field
read - `E0609`), the `readdir` member is the mirror's untyped `*mut c_void` so the assignment needs
`std::mem::transmute` (`E0605` forbids the `as` cast between function-pointer types; the
`femacs/src/sysdep.rs` 2873 spelling), `DT_DIR`/`DT_REG` are not in `lib/include/dirent.rs`
(declared at the use site, measured 4/8 with this machine's NDK), and `AAsset_close` is the ninth
name of the arm-split `<android/asset_manager.h>` family, so the `>= 9` arm declares it (arm B alone
read `E0425` for it - the arm work the slice-4/5 Deviations 17/18 describe, done before the probe
for the five names this round needs: `AASSET_MODE_STREAMING`, `AAsset_close`, `DT_DIR`, `DT_REG`,
`emacs_installation_time`).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 1883's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 167 -> 149 rows, diffed**: the
eighteen `missing function android_afs_*` rows retire, three `does not mention` rows appeared and
are now discharged by `docs/ports/macro-body-notes.tsv` (the `goto`/labelled-block rows for
`android_afs_name` and `android_afs_stat`, and the `#ifdef STAT_TIMESPEC` pair's six tokens for
`android_afs_open`/`android_afs_stat`), and the whole-file line-count row moves 69.3% -> **57.7%**
(C 7,950 against 3,364 Rust lines).

**Deviations (this slice's additions).**

21. **`afs_vfs_ops`'s text sits out of C's order, at the unit's foot** (the P-91 clique Deviation 20
    named): the object's initialiser names thirteen functions C defines after it, and two of them
    (`android_afs_name` C 1740, `android_afs_initial` C 1834) name the object back, so the three
    land in one round with the object moved to the round's end - the haikuterm slice-22 device
    (its Deviation 34).  Nothing is reordered *within* a landed range.
22. **Three classes of name the mirrors do not carry are declared where they are used**: `struct
    dirent` (`lib/include/dirent.rs` 6 says "not mirrored yet - no ported unit reads its fields";
    this file is the first, so the struct and its measured layout stand here), `DT_DIR`/`DT_REG`
    (the same mirror keeps only gnulib's `_GL_DT_NOTDIR`), and `emacs_installation_time` (android.h
    338; `include/android.rs` 238 carries it, but that mirror is not in this unit's include graph).
    Plus the `#ifdef STAT_TIMESPEC` pair's treatment: the live arm reads the mirror's `st_mtimespec`
    (`STAT_TIMESPEC (st, st_mtim)` is `stat-time.h` 61's `##`-composed member name), and the `#else`
    arm is **carried as a record**, because its `st_mtime_nsec` exists in no mirror - the Owed item
    below names the mirror change that would let it be transcribed.

**Owed (this slice's additions).**

- **`lib/include/sys_stat.in.rs` needs the gnulib `stat.h` override's nanosecond fields** (at least
  `st_mtime_nsec` under the C condition that names it) for C 1985-1986 and C 2167-2168's arms to be
  transcribed rather than recorded; the two records are the only text this file does not carry.
- The new cfg feature `stat_timespec` (C's `STAT_TIMESPEC`, live on this target) is not in
  `docs/baseline/feature-coverage.tsv`; the integrator's `--write` re-run adds it (the
  `android_api_ge_9` item's class, Owed 1).
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **3,364** of 7,950 C lines (42.3%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; two of the five rows - C 1980 and C 2162 - are
  now *inside* the ported prefix and are this round's Deviation 22).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No asset was opened, no directory
stream was created, `afs_file_descriptors`/`all_afs_vdirs` are still null, and `afs_vfs_ops` has
never been called through.  What the round's evidence *is*: both arms' `exit 0`, one control proving
the harness reads this text, `--check` 167 -> 149 with the row set diffed, the measured layouts and
constants, and the ten gates.  **This file is not finished**: C 2470-7950 remains (the next item is
C 2470's `struct android_content_vdir`).

## Slice 9 (2026-10-08, status 4.114 - the integrator sets the round number): C 2470-2946 except the three deferred items (the content family's 15 functions and the JNI entry point)

**What this round is.**  `struct android_content_vdir` (C 2470-2484) with its measured layout, C's
declarations (C 2486-2518, no Rust text), the meta-interface comment (C 2490-2500),
`content_directory_contents` and `all_content_vdirs` (C 2539-2548), the fifteen content functions
(`android_content_open` … `android_content_get_directory_name`, C 2641-2885), the `#pragma` region
carried as a record (C 2908-2916/2942-2946) and `displayNameHash` (C 2920-2940, the JNI entry point
whose symbol `NATIVE_NAME` pastes).  Port: **3,364 -> 3,758 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors, 7 warnings in the else arm and 12 in the `>= 9`
arm (the same counts as the previous round).  One real defect came out of the first draft:
`&raw mut content_directory_contents as *mut *const c_char.offset (…)` is `E0689` ("cast cannot be
followed by a method call"), so the cast is parenthesised before the `.offset`.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 2768's
`dir = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 149 -> 133 rows, diffed**: sixteen
`missing function android_content_*` rows and `missing file-scope object all_content_vdirs` retire,
**one row appears** - `fn displayNameHash carries #[unsafe(no_mangle)] and exports
`Java_org_femacs_EmacsNative_displayNameHash`, but the reference does not define a non-static
function of either name (invented exported symbol)` - which is the class `docs/ports/android.md`
432/462 already records three times for `android.rs` (the `NATIVE_NAME` paste, the same Owed 9 blind
spot there), and the whole-file line-count row moves 57.7% -> **52.7%**.

**Deviation (this slice's addition).**

23. **The content object, `android_content_name` and `android_content_initial` are one clique and are
    deferred together** (Deviation 20/21's rule, applied one family later): `content_vfs_ops` names
    `android_content_name`; `android_content_name`'s function-scope `content_vnodes[]` names
    `android_authority_initial` (C 3577), `android_authority_initial_name` (C 3603) and
    `android_saf_root_initial` (C 4219); and `android_content_initial` names `content_vfs_ops`.  So
    the three land in the round that reaches C 3577, with the object at that round's foot, and this
    round's text records where C's text would be.  The three rows in the range table are `deferred`.

**Owed (this slice's additions).**

- The three deferred ranges (C 2520-2537, C 2549-2639, C 2887-2906) plus C 2956-3206's four helpers
  (`android_get_content_name`, `android_check_content_access`, `android_saf_exception_check`,
  `android_saf_check_nonnull`) are the next round's first items - the four are deferrable to it
  because the content family never calls them (their first callers are C 3314 onward).
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **3,758** of 7,950 C lines (47.3%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No content vnode was created, no
directory stream was opened, `all_content_vdirs` is still null, and `displayNameHash` has never been
called from Java.  What the round's evidence *is*: both arms' `exit 0`, one control proving the
harness reads this text, `--check` 149 -> 133 with the row set diffed, the measured layout, and the
ten gates.  **This file is not finished**: C 2956-7950 remains (the next item is C 2956's
`android_get_content_name`).

## Slice 10 (2026-10-08, status 4.115 - the integrator sets the round number): C 2948-3195 (the two URI helpers and the two variadic SAF checkers)

**What this round is.**  `android_get_content_name` (C 2956-3034, C's `goto no_checksum` as the
port's labelled block), `android_check_content_access` (C 3041-3066),
`android_saf_exception_check` (C 3095-3164) and `android_saf_check_nonnull` (C 3171-3195) with their
comments.  Port: **3,758 -> 4,129 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm (one more each than the previous round, and the else arm's list was printed and diffed: the extra
one is the FFI-safety note for the new `android_emacs_service` declaration, i.e. ``\`extern\` block
uses type \`_jmethodID\`, which is not FFI-safe``).  Two real defects came out of the first drafts:
⑴ `unsafe fn … (…, ...)` is ``\`...\` is not supported for non-extern functions`` - a variadic
*definition* has to be `extern "C"` (the `lib/fcntl.rs` 142 precedent), which is also C's own ABI
here; ⑵ `CallNonvirtualBooleanMethod (…, read, write)` is `E0617 can't pass \`u8\` to variadic
function`, because C's own call promotes the two `jboolean`s to `int` - the port writes
`(read as c_int)` / `(write as c_int)`, the slice-5 `open` call's class.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 2967's
`strchr` call moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm
reads `exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 133 -> 129 rows, diffed**: four
`missing function` rows retire (`android_get_content_name`, `android_check_content_access`,
`android_saf_exception_check`, `android_saf_check_nonnull`); eight rows appeared - four
`does not mention` rows and four `va-arg` rows - and **all eight are discharged in this same
commit**: the four body rows are written into `docs/ports/macro-body-notes.tsv` (the
`android_get_content_name` `goto` row, the `android_check_content_access`
`android_delete_local_ref` row, and the two checkers' `android_delete_local_ref,va_end,va_list,
va_start` rows) and the four `va-arg` rows by one `jobject` row in `docs/ports/va-arg-types.tsv`.
The whole-file line-count row moves 52.7% -> **48.1%** (C 7,950 against 4,129 Rust lines).

**Deviations (this slice's additions).**

24. **A variadic C definition is an `extern "C"` Rust function, and its `va_arg` reads are
    `next_arg::<T> ()`** (Evidence ①): C's `va_list ap; va_start (ap, n); … va_end (ap);` has no port
    counterpart, the `...` parameter *is* the list, and the sole read's C type (`jobject`) is
    recorded in `docs/ports/va-arg-types.tsv` (the `fcntl` rule, P-04).  C calls both checkers
    directly, so their ABI is chosen by the variadic definition rather than by the address-taken
    rule - a case the `android_unix_vnode` rule does not cover.
25. **The Java-side service table and five more names stand at the use site** (P-72 rule 4): the
    `android.h` mirror `include/android.rs` carries `android_java_env` (321), `emacs_service` (331),
    `service_class` (334), `android_exception_check` (129), `android_exception_check_1` (130) and
    `struct android_emacs_service` (275-319), but the edge that would put it in this unit's graph is
    Owed 2 (its `android_vdir` stand-in is deleted when that edge lands), so this round declares all
    six - the struct with C's own 42 fields and its measured 336-byte layout - and the declaration
    retires with the mirror edge.

**Owed (this slice's additions).**

- The deferred clique (C 2520-2537, C 2549-2639, C 2887-2906) is still the round that reaches
  C 3577's first item.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **4,129** of 7,950 C lines (51.9%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No content URI was built, no JNI
exception was checked, and the two checkers have never been called with an argument list.  What the
round's evidence *is*: both arms' `exit 0`, one control proving the harness reads this text,
`--check` 133 -> 129 with all eight new rows discharged in-commit, the measured layout, and the ten
gates.  **This file is not finished**: C 3197-7950 remains (the next item is C 3199's comment on the
authority family and C 3208's `struct android_authority_vnode`).

## Slice 11 (2026-10-08, status 4.116 - the integrator sets the round number): C 3197-3613 (the by-authority family)

**What this round is.**  `struct android_authority_vnode` (C 3208-3216) with its measured layout, C's
twelve forward declarations (C 3218-3234, no Rust text), `android_authority_name` (C 3255-3335, C's
`goto no_entry`/`goto error` and the two labels as the port's labelled block plus the written-out
`error:` path), `android_authority_open` (C 3337-3395), `android_authority_close`, the nine small
members (C 3410-3561), `android_authority_initial` (C 3576-3587) and
`android_authority_initial_name` (C 3602-3613), with `authority_vfs_ops` (C 3238-3253) at the file's
foot.  Port: **4,129 -> 4,614 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous round read, so this round's text adds no warning.  Two real
defects came out of the first drafts, both one class: C's `mode & O_WRONLY` / `& O_RDWR` /
`& O_TRUNC` / `& O_CLOEXEC` (C 3371-3376, C 3390) mix a `mode_t` with an `int` - `E0277 no
implementation for \`u16 & i32\`` and five `E0308`s - because the mirror types `mode_t` as the frozen
target's 16-bit unsigned; the port writes `(mode as c_int)`, which is exactly C's own integer
promotion (the `android_afs_open` 1190 `open` call's class).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 3271's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 129 -> 114 rows, diffed**: fifteen
`missing function android_authority_*` rows retire, two `does not mention` rows appear and are
discharged in this same commit (`docs/ports/macro-body-notes.tsv` gains the
`android_authority_name` `error,goto` row and the `android_authority_open`
`android_delete_local_ref,jboolean` row), and the whole-file line-count row moves 48.1% -> **42.0%**
(C 7,950 against 4,614 Rust lines).

**Deviation (this slice's addition).**

26. **`authority_vfs_ops`'s text sits out of C's order, at the unit's foot** - the same P-91 device
    Deviation 21 records for `afs_vfs_ops`: the object names the twelve `android_authority_*`
    functions C defines after it and Rust cannot forward-declare, so the object's text moves to the
    end of the round that lands the last of them.  Nothing is reordered *within* a landed range, and
    the range table's row says which item it is.

**Owed (this slice's additions).**

- The deferred clique (C 2520-2537, C 2549-2639, C 2887-2906) is still open - **the round that
  reaches C 4219** closes it (the earlier note's C 3577 was wrong: `content_vnodes[]`'s third entry
  is `android_saf_root_initial`, C 4219).
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **4,614** of 7,950 C lines (58.0%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No authority vnode was created, no
content URI was opened, and `authority_vfs_ops` has never been called through.  What the round's
evidence *is*: both arms' `exit 0`, one control proving the harness reads this text, `--check`
129 -> 114 with both new rows discharged in-commit, the measured layout, and the ten gates.  **This
file is not finished**: C 3615-7950 remains (the next item is C 3638's `struct android_saf_root_vnode`
under C 3617-3636's comment).

## Slice 12 (2026-10-08, status 4.117 - the integrator sets the round number): C 3617-4246 except the three SAF-root items the C 5773 chain holds

**What this round is.**  `struct android_saf_root_vnode` (C 3638-3646) and `struct
android_saf_root_vdir` (C 3648-3668) with their measured layouts, C's twelve forward declarations
(C 3670-3686, no Rust text), `all_saf_root_vdirs` (C 3708), C 3710-3712's forward declaration of
`android_saf_tree_from_name`, `android_saf_valid_authority_p` (C 3714-3746), the fifteen SAF root
members (C 3850-4207: open, close, unlink, symlink, rmdir, rename, stat, access, mkdir, chmod,
readlink, readdir, closedir, dirfd, opendir) and `android_saf_root_get_directory` (C 4234-4246).
Port: **4,614 -> 5,215 lines**.

**What this round cannot land, and why (Deviation 27).**  `android_saf_root_name` (C 3748-3848)
calls `android_saf_tree_from_name` (C 5773, "defined in the next page"), `android_content_initial`
(C 3843) and `android_saf_root_initial` (C 3847); `android_saf_root_initial` (C 4209-4229) calls
`android_saf_root_name`; `saf_root_vfs_ops` (C 3688-3705) names `android_saf_root_name`; and the
content clique's `android_content_name` (C 2549) names `android_saf_root_initial` through its
`content_vnodes[]`.  All six are one chain that closes only at **C 5773**, so this round records
three more `deferred` rows and the earlier rows' target is corrected (their C 4219 was wrong).

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous two rounds read, so this round's text adds no warning.  One
real defect came out of the first draft: `android_exception_check_nonnull` (C 4025) had no
declaration in this unit - `E0425 cannot find function android_exception_check_nonnull in this
scope` in **both** arms - so the slice-10 use-site block gained android.h 138's prototype.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 3917's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 114 -> 96 rows, diffed**: seventeen
`missing function android_saf_root_*` rows and `missing file-scope object all_saf_root_vdirs` retire
(plus `missing function android_saf_valid_authority_p`); four `does not mention` rows appear - the
`android_delete_local_ref` class in `android_saf_valid_authority_p`, `android_saf_root_readdir`,
`android_saf_root_closedir` and `android_saf_root_opendir` - and all four are discharged in this same
commit by rows in `docs/ports/macro-body-notes.tsv`.  The whole-file line-count row moves 42.0% ->
**34.4%** (C 7,950 against 5,215 Rust lines).

**Owed (this slice's additions).**

- The six-item chain above is the round that reaches C 5773's first work.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **5,215** of 7,950 C lines (65.6%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No SAF root vnode was created, no
document-tree array was fetched, `all_saf_root_vdirs` is still null, and `saf_root_vfs_ops` has never
been called through.  What the round's evidence *is*: both arms' `exit 0`, one control proving the
harness reads this text, `--check` 114 -> 96 with all four new rows discharged in-commit, the
measured layouts, and the ten gates.  **This file is not finished**: C 4248-7950 remains (the next
item is C 4250's comment on the SAF directory/file nodes and C 4254's `inside_saf_critical_section`).

## Slice 13 (2026-10-08, status 4.118 - the integrator sets the round number): C 4248-4572 (the functions the SAF directory and file nodes share)

**What this round is.**  `inside_saf_critical_section` (C 4254), `android_saf_stat` (C 4265-4358,
with C 4346-4354's `#ifdef STAT_TIMESPEC` pair), `android_saf_access` (C 4371-4445, C's `switch` as
the port's `match`), `android_saf_delete_document` (C 4456-4497), C 4499-4501's forward declaration of
`android_document_id_from_name` (no Rust text) and `android_saf_rename_document` (C 4515-4572), with
their comments.  Port: **5,215 -> 5,658 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous three rounds read, so this round's text adds no warning.  Two
real defects came out of the first draft: `TYPE_MAXIMUM! (off_t)` (C 4343) had no macro in this
unit's scope - `cannot find macro TYPE_MAXIMUM in this scope` in both arms - so the unit declares it
at the use site with `<intprops.h>`'s name and body-value (Deviation 28); and C 4304/4408's
`(jboolean) …` arguments of the two variadic JNI calls are C's own promotion, so the port writes
`as c_int` (slice 10's rule, applied twice more).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 4284's
`NewStringUTF` call moves the else arm to `exit 1` with `injected_control` named once; reverted, the
arm reads `exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 96 -> 91 rows, diffed**: five
`missing` rows retire (`inside_saf_critical_section`, `android_saf_stat`, `android_saf_access`,
`android_saf_delete_document`, `android_saf_rename_document`); four `does not mention` rows appear
and all four are discharged in this same commit by rows in `docs/ports/macro-body-notes.tsv`; and the
whole-file line-count row moves 34.4% -> **28.8%** (C 7,950 against 5,658 Rust lines).

**Deviations (this slice's additions).**

28. **`TYPE_MAXIMUM` stands at the use site, and `android_saf_move_document` waits for its
    neighbour**: `<intprops.h>`'s macro is carried by `lib/include/intprops.rs` 80/96, which this
    unit does not include, so the name is declared here with C's name and (for the one type the call
    site uses) that body's value - the P-72-rule-4 device the `android_cache_dir` /
    `android_emacs_service` / `assert` declarations already use.  And C 4574-4677's
    `android_saf_move_document` calls C 4806's `android_document_id_from_name` (whose forward
    declaration C writes at C 4499-4501), so it lands with the SAF directory family rather than
    here; its range-table row is `deferred`.

**Owed (this slice's additions).**

- The eight deferred items (the six-item chain plus `android_saf_move_document`) are the next round's
  first work, together with C 4806's `android_document_id_from_name`.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **5,658** of 7,950 C lines (71.2%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item; C 4346's `#ifdef STAT_TIMESPEC` is one of them).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No document status was fetched, no
document was deleted or renamed, and `inside_saf_critical_section` has never been set by a real SAF
operation.  What the round's evidence *is*: both arms' `exit 0`, one control proving the harness
reads this text, `--check` 96 -> 91 with all four new rows discharged in-commit, and the ten gates.
**This file is not finished**: C 4679-7950 remains (the next item is C 4681's comment on the SAF
directory family and C 4687's `struct android_saf_tree_vnode`).

## Slice 14 (2026-10-08, status 4.119 - the integrator sets the round number): C 4574-4895 (the deferred move, the SAF tree structs and the document-ID lookup)

**What this round is.**  `android_saf_move_document` (C 4584-4677) - the item slice 13 deferred,
which this round can land because the function it calls arrives with it - then
`struct android_saf_tree_vnode` (C 4687-4712) and `struct android_saf_tree_vdir` (C 4714-4732) with
their measured layouts, C's twelve forward declarations (C 4734-4750, no Rust text), C 4771-4777's
two tentative vector declarations under their own comments, `all_saf_tree_vdirs` (C 4780) and
`android_document_id_from_name` (C 4805-4895).  Port: **5,658 -> 6,040 lines**.

**Evidence ① - both arms read `exit 0` on the first draft**: 0 errors; 8 warnings in the else arm and
13 in the `>= 9` arm - the same two counts the previous four rounds read, so this round's text adds
no warning and needed no repair pass.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 4621's
`id = *doc_id;` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm
reads `exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 91 -> 88 rows, diffed**: three
`missing` rows retire (`missing file-scope object all_saf_tree_vdirs`, `missing function
android_document_id_from_name`, `missing function android_saf_move_document`); two `does not mention`
rows appear (the `android_delete_local_ref` class in `android_document_id_from_name` and, with the
`switch` tokens, in `android_saf_move_document`) and both are discharged in this same commit by rows
in `docs/ports/macro-body-notes.tsv`.  The whole-file line-count row moves 28.8% -> **24.0%**
(C 7,950 against 6,040 Rust lines).

**Deviations (this slice's additions).**

29. **One more tentative vector declaration is a record, and `saf_tree_vfs_ops` waits one round.**
    C 4771-4777 declares `saf_file_vfs_ops` and `saf_new_vfs_ops` as *tentative definitions* whose
    initialisers C writes two pages later ("Defined later in the next page" / "two pages below"); the
    port records both with their comments and lands them where C's initialisers are, because a
    Rust `static` needs its initialiser at the declaration (the `android_afs_vname`-family
    tentative-definition rule's other edge).  And `saf_tree_vfs_ops` (C 4752-4769) names
    `android_saf_tree_name` (C 4898) and the eleven members after it, so the object lands at the foot
    of the round that reaches C 5480 - the P-91 device Deviations 21/26 use.

**Owed (this slice's additions).**

- The seven deferred items (the six-item chain plus `saf_tree_vfs_ops`) are the next rounds' first
  work: `saf_tree_vfs_ops` closes at C 5480, the six-item chain at C 5773.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **6,040** of 7,950 C lines (76.0%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No document ID was looked up, no
document was moved, and `all_saf_tree_vdirs` is still null.  What the round's evidence *is*: both
arms' `exit 0` on the first draft, one control proving the harness reads this text, `--check`
91 -> 88 with both new rows discharged in-commit, the measured layouts, and the ten gates.  **This
file is not finished**: C 4897-7950 remains (the next item is C 4898's `android_saf_tree_name`).

## Slice 15 (2026-10-08, status 4.120 - the integrator sets the round number): C 5114-5529 (the SAF tree members except `name`, and the Cursor opener)

**What this round is.**  The twelve SAF tree members that do not name the deferred vectors -
`android_saf_tree_open` (C 5114-5122), `close` (C 5124-5139), `unlink` (C 5141-5146), `symlink`
(C 5148-5153), `rmdir` (C 5155-5173), `rename` (C 5176-5403, the round's largest item, with C's five
`goto error` sites and the `error:` label), `stat` (C 5405-5415), `access` (C 5417-5434), `mkdir`
(C 5436-5443), `chmod` (C 5445-5461), `readlink` (C 5463-5471) and `opendir_1` (C 5479-5529).  Port:
**6,040 -> 6,517 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous five rounds read, so this round's text adds no warning.  One
real defect came out of the first draft, and it was a *control-flow* one rather than a type error:
`android_saf_tree_rename`'s `error:` label is inside C's second case, and the label's two statements
(`xfree (dst_id); return 1;`) sit after the four `goto error` sites - a first draft that put the
whole `if` inside one labelled block left the error path falling through into the simple-rename tail
instead.  The repair is the shape this ledger records (a labelled block *inside* the case, with the
label's statements after it), and measured: the two arms stayed at 8/13 warnings, so the reading did
not move while the text did.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 5196's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 88 -> 76 rows, diffed**: twelve
`missing function android_saf_tree_*` rows retire; two `does not mention` rows appear
(`android_saf_tree_opendir_1` → `android_delete_local_ref`, and `android_saf_tree_rename` →
`case,default,goto,switch`) and both are discharged in this same commit by rows in
`docs/ports/macro-body-notes.tsv`.  The whole-file line-count row moves 24.0% -> **18.0%** (C 7,950
against 6,517 Rust lines).

**Deviation (this slice's addition).**

30. **`android_saf_tree_name` waits for the file/new families, so the earlier target was one page
    short.**  C 4967/5007 name `saf_new_vfs_ops` and `saf_file_vfs_ops`, the two *tentative* vectors
    C 4771-4777 declares; a Rust `static` needs its initialiser where it is written, so those two
    objects can only be defined in the rounds that write the file and ``new'' families' own
    functions - which means `android_saf_tree_name` (and `saf_tree_vfs_ops`, whose last member is
    C 5686's `android_saf_tree_opendir`) land there rather than with this round.  The two `deferred`
    rows say so, and the earlier row's C 5480 is corrected to C 5686.

**Owed (this slice's additions).**

- The deferred set is now: the six-item chain (closing at C 5773), `saf_tree_vfs_ops` (C 5686),
  `android_saf_tree_name` (C 4897-5112) and `android_saf_tree_from_name` (C 5773-5846) - the last two
  with the file/new families' vectors.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **6,517** of 7,950 C lines (82.0%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No document was renamed or deleted, no
cursor was opened, and `all_saf_tree_vdirs` is still null.  What the round's evidence *is*: both arms'
`exit 0`, one control proving the harness reads this text, `--check` 88 -> 76 with both new rows
discharged in-commit, and the ten gates.  **This file is not finished**: C 5531-7950 remains (the next
item is C 5531's `android_saf_tree_readdir`).

## Slice 16 (2026-10-08, status 4.121 - the integrator sets the round number): C 5531-5863 (the SAF tree's directory stream, and the stream lookup)

**What this round is.**  `android_saf_tree_readdir` (C 5531-5626, with C 5594's
`coding.mode |= CODING_MODE_LAST_BLOCK` as the mirror's bitfield write and C 5589-5598's
`decode_coding_object` chain), `closedir` (C 5628-5662), `dirfd` (C 5664-5685), `opendir`
(C 5686-5759) and `android_saf_tree_get_directory` (C 5851-5863), with their comments, plus the
use-site declarations this round needs (`setup_coding_system`, coding.h 686; `asprintf`,
`<stdio.h>`) and `DT_UNKNOWN` (measured 0).  Port: **6,517 -> 6,846 lines**.

**Evidence ① - both arms read `exit 0` on the first draft**: 0 errors; 8 warnings in the else arm and
13 in the `>= 9` arm - the same two counts the previous six rounds read.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 5543's
`dir = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 76 -> 71 rows, diffed**: five
`missing function android_saf_tree_*` rows retire (`readdir`, `closedir`, `dirfd`, `opendir`,
`get_directory`); two `does not mention` rows appear (`android_saf_tree_readdir` →
`android_delete_local_ref,mode`, `android_saf_tree_closedir` → `android_delete_local_ref`) and both
are discharged in this same commit by rows in `docs/ports/macro-body-notes.tsv`.  The whole-file
line-count row moves 18.0% -> **13.9%** (C 7,950 against 6,846 Rust lines).

**Owed (this slice's additions).**

- `android_saf_tree_from_name` (C 5761-5846) joins the deferred set, which is now: the six-item chain
  and `saf_tree_vfs_ops`/`android_saf_tree_name`/`android_saf_tree_from_name`, all closing in the
  round that writes the SAF file and ``new'' families' functions (the two tentative vectors'
  initialisers).
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **6,846** of 7,950 C lines (86.1%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No directory entry was read from a
Cursor, no coding system was set up, and `all_saf_tree_vdirs` is still null.  What the round's
evidence *is*: both arms' `exit 0` on the first draft, one control proving the harness reads this
text, `--check` 76 -> 71 with both new rows discharged in-commit, and the ten gates.  **This file is
not finished**: C 5878-7950 remains (the next item is C 5878's `struct android_parcel_fd`).

## Slice 17 (2026-10-08, status 4.122 - the integrator sets the round number): C 5876-6200 (the SAF file family, and the first tentative vector's initialiser)

**What this round is.**  `struct android_parcel_fd` (C 5878-5892) with its measured layout, C
5894-5900's five forward declarations (no Rust text), **`saf_file_vfs_ops` (C 5902-5919) - the
object C 4773 declared as a tentative definition, now written with its initialiser**,
`open_parcel_fds` (C 5922), C 5871-5874's `#define android_saf_file_vnode android_saf_tree_vnode`
alias as the port's `type` alias, `android_saf_file_name` (C 5924-5950), `android_saf_file_open`
(C 5952-6106, the round's largest item: the three `jboolean` access-mode folds, `NewGlobalRef` and
its `EMFILE` failure arm, and the `get_stat_mtime`/`invalid_timespec` mtime probe), `_unlink`
(C 6108-6117), `_rmdir` (C 6119-6124), `_opendir` (C 6126-6131) and `android_close_parcel_fd`
(C 6140-6170).  Port: **6,846 -> 7,219 lines**.

**Evidence ① - both arms read `exit 0` on the first draft**: 0 errors; 8 warnings in the else arm and
13 in the `>= 9` arm - the same two counts the previous seven rounds read.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 5982's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 71 -> 63 rows, diffed**: seven
`missing` rows retire (`missing file-scope object open_parcel_fds`, `missing file-scope object
saf_file_vfs_ops`, `missing function android_close_parcel_fd` and the five `android_saf_file_*`
functions); one `does not mention` row appears (`android_saf_file_open` →
`android_delete_local_ref`) and is discharged in this same commit.  The whole-file line-count row
moves 13.9% -> **9.2%** (C 7,950 against 7,219 Rust lines).

**Deviation (this slice's addition).**

31. **The first tentative vector is now a real object, and the second one's alias waits.**  C 5902's
    `saf_file_vfs_ops` initialiser is written where C writes it, because all thirteen of its members
    - five from this family and eight from the SAF tree family - are in the unit by the end of this
    round; that is the difference from `afs_vfs_ops`/`authority_vfs_ops`, whose cliques forced their
    text to the file's foot (Deviations 21/26).  C 6183's `#define android_saf_new_vnode
    android_saf_tree_vnode` is the same kind of alias as C 5874's, and it lands with its family next
    round.

**Owed (this slice's additions).**

- The deferred set is now: the six-item chain, `saf_tree_vfs_ops`, `android_saf_tree_name`,
  `android_saf_tree_from_name` and the ``new'' family's alias - and `saf_new_vfs_ops` (C 6204), whose
  initialiser names that family's functions, is next round's first item.
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **7,219** of 7,950 C lines (90.8%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No parcel descriptor was opened, no
global reference was made, and `open_parcel_fds` is still null.  What the round's evidence *is*: both
arms' `exit 0` on the first draft, one control proving the harness reads this text, `--check`
71 -> 63 with the new row discharged in-commit, the measured layout, and the ten gates.  **This file
is not finished**: C 6202-7950 remains (the next item is C 6204's `saf_new_vfs_ops` and C 6222's
`android_saf_new_name`).

## Slice 18 (2026-10-08, status 4.123 - the integrator sets the round number): C 6172-6500 (the SAF ``new'' family, minus the one member the closure round holds)

**What this round is.**  The SAF ``new'' vnodes' comment (C 6174-6181), C 6183's
`#define android_saf_new_vnode android_saf_tree_vnode` alias as a Rust `type` alias, and eleven of
the family's twelve members: `android_saf_new_name` (C 6221-6247), `_open` (C 6249-6339, which
transforms the vnode into a file vnode and calls through its own `open`), `_unlink` (C 6341-6346),
`_symlink` (C 6348-6353), `_rmdir` (C 6355-6360), `_rename` (C 6362-6369), `_stat` (C 6371-6377),
`_access` (C 6379-6388), `_chmod` (C 6479-6485), `_readlink` (C 6487-6493) and `_opendir`
(C 6495-6500).  Port: **7,219 -> 7,480 lines**.

**Evidence ① - both arms read `exit 0` on the first draft**: 0 errors; 8 warnings in the else arm and
13 in the `>= 9` arm - the same two counts the previous eight rounds read.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 6270's
`vp = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 63 -> 52 rows, diffed**: eleven
`missing function android_saf_new_*` rows retire; one `does not mention` row appears
(`android_saf_new_open` → `android_delete_local_ref`) and is discharged in this same commit.  The
whole-file line-count row moves 9.2% -> **5.9%** (C 7,950 against 7,480 Rust lines).

**Deviation (this slice's addition).**

32. **The SAF ``new'' vector and its `mkdir` member are held back one round, and they are what the
    closure round needs most.**  `saf_new_vfs_ops` (C 6204-6219) names `android_saf_new_mkdir`
    (C 6390-6477), and that function's C 6475 writes `vp->vnode.ops = &saf_tree_vfs_ops`; that object
    in turn names `android_saf_tree_name` (C 4897), which names `saf_file_vfs_ops` and
    `saf_new_vfs_ops`.  The loop closes *through* the round that writes all of them, so this round
    lands everything that is not on the cycle and leaves the two objects plus `android_saf_new_mkdir`
    to the closure round - which is also where the six-item chain and
    `android_saf_tree_from_name`/`android_saf_tree_name` land.

**Owed (this slice's additions).**

- The closure round's set is now explicit: `saf_new_vfs_ops` (C 6204), `android_saf_new_mkdir`
  (C 6390), `saf_tree_vfs_ops` (C 4752), `android_saf_tree_name` (C 4897), `android_saf_tree_from_name`
  (C 5761), `saf_root_vfs_ops` (C 3688), `android_saf_root_name` (C 3748),
  `android_saf_root_initial` (C 4209), `content_vfs_ops` (C 2520), `android_content_name` (C 2549)
  and `android_content_initial` (C 2887).
- `docs/baseline/port-lines-pending.tsv` (Owed 5) stays stale: **7,480** of 7,950 C lines (94.1%).
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  No document was created, no vnode was
transformed, and the ``new'' vector has never been called through.  What the round's evidence *is*:
both arms' `exit 0` on the first draft, one control proving the harness reads this text, `--check`
63 -> 52 with the new row discharged in-commit, and the ten gates.  **This file is not finished**: C
6507-7950 remains (the next item is C 6508's `saf_completion_sem`).

## Slice 19 (2026-10-08, status 4.124 - the integrator sets the round number): the closure round - eleven deferred items, C 1-6500 now carries no gap

**What this round is.**  Every item this lane had deferred, in one round: `content_vfs_ops`
(C 2520-2537) with `android_content_name` (C 2549-2639) and `android_content_initial`
(C 2887-2906); `saf_root_vfs_ops` (C 3688-3705) with `android_saf_root_name` (C 3748-3848) and
`android_saf_root_initial` (C 4209-4229); `saf_tree_vfs_ops` (C 4752-4769) with
`android_saf_tree_name` (C 4897-5112, the largest of them) and `android_saf_tree_from_name`
(C 5761-5846); and `saf_new_vfs_ops` (C 6204-6219) with `android_saf_new_mkdir` (C 6390-6477).
Port: **7,480 -> 8,271 lines**.

**Why they could not be split (Deviations 23/27/29/30/32, now closed).**  The eleven names form one
cycle: `android_content_name`'s `content_vnodes[]` needs `android_saf_root_initial`;
`android_saf_root_name` needs `android_saf_tree_from_name`; that needs `saf_tree_vfs_ops`; that
names `android_saf_tree_name`; that needs `saf_file_vfs_ops` **and** `saf_new_vfs_ops`; the latter
names `android_saf_new_mkdir`; and its C 6475 writes `saf_tree_vfs_ops` again.  Rust has no forward
declaration (PORTING.md 10.10), so a round that lands part of the cycle cannot type-check - which is
why slices 7-18 recorded rather than wrote these items, and why this round writes all eleven at
once.  Unlike `afs_vfs_ops`/`authority_vfs_ops` (Deviations 21/26), none of the four objects needs
its text moved to the file's foot: item order in Rust is free, so each sits where C puts it.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous nine rounds read.  Three real defects came out of the first
drafts: `special_coding_system: 0` is `E0308` (the field is the mirror's `*mut Lisp_X`, so the C
initialiser's trailing comma is `std::ptr::null_mut ()`); `*name.offset ((*special).length) as isize`
parses as `*name.offset(…) as isize` and the comparison wants `c_char`, so the cast is parenthesised
(`E0308` twice); and `android_saf_tree_name` assigns its own `vnode` parameter twice (C 5073/5108),
which is `E0384 cannot assign to immutable argument` until the binding is `mut`.

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 2560's
`content_vnodes[]` moves the else arm to `exit 1` with `injected_control` named once; reverted, the
arm reads `exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 52 -> 43 rows, diffed**: nine rows
retire - the eight `missing function`/`missing file-scope object` rows of the deferred set **and the
whole-file line-count row**, which retires because the port's 8,271 lines are now within the
checker's ±5% of the reference's 7,950 (it read 5.9% before this round and 13.9% two rounds ago);
five `does not mention` rows appear (three `goto` rows for the two labelled-block functions'
families, and the `android_delete_local_ref`/`jstring` rows of `android_saf_tree_from_name` and
`android_saf_new_mkdir`) and all five are discharged in this same commit by rows in
`docs/ports/macro-body-notes.tsv`.

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5): this file's row is stale and now reads over
  100% - **8,271** Rust lines against 7,950 C lines; the integrator's `--write-pending` re-run drops
  it.
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  Nothing in the eleven items ever ran:
no document ID was looked up through `android_saf_tree_name`, no content URI was built, no SAF root
listing was fetched, and none of the four vectors has been called through.  What the round's
evidence *is*: both arms' `exit 0` on the closure of a nine-round-old deferral set, one control
proving the harness reads this text, `--check` 52 -> 43 with all five new rows discharged in-commit,
and the ten gates.  **This file is not finished**: C 6507-7950 remains (the next item is C 6508's
`saf_completion_sem`).

## Slice 20 (2026-10-08, status 4.125 - the integrator sets the round number): C 6507-6852 (the SAF synchronization half and the root vnode family)

**What this round is.**  `saf_completion_sem` (C 6508) with the `<semaphore.h>` declarations it
needs, the two `#pragma` pairs as records, the four JNI entry points (C 6518-6571:
`safSyncAndReadInput`, `safSync`, `safPostRequest` and `ftruncate`), and then the root family:
`struct android_root_vdir` (C 6587-6597) with its measured layout, `root_fd`/`root_fd_references`
(C 6601/6604), `root_vfs_ops` (C 6613-6628), `special_vnodes` (C 6632-6637, whose second entry
carries `LISPSYM_INITIALLY (Qandroid_jni)`), `android_vfs_convert_name` (C 6649-6659),
`android_root_name` (C 6661-6766, with its two `USE_SAFE_ALLOCA` blocks), `android_root_readdir`
(C 6768-6789), `android_root_closedir` (C 6791-6811), `android_root_dirfd` (C 6813-6818) and
`android_root_opendir` (C 6820-6852).  Port: **8,271 -> 8,730 lines**.

**Evidence ① - both arms read `exit 0`**: 0 errors; 8 warnings in the else arm and 13 in the `>= 9`
arm - the same two counts the previous ten rounds read.  Three real defects came out of the first
drafts, all of them the same class this ledger has recorded before: `JNI_STACK_ALIGNMENT_PROLOGUE`
and the four JNI entries' `NATIVE_NAME` pastes needed their names in this unit's scope (the macro
now stands at the use site with the `android.h` mirror's own no-op body, and the four entries export
the port's own class's symbol), `process_pending_signals` (lisp.h 3880) and `code_convert_string`
(coding.h 689) needed use-site declarations, `ANDROID_LOG_VERBOSE` needed its measured value (2),
and `name1 = 0` was `E0308` because `Lisp_Object` is the mirror's `*mut Lisp_X` (the
`special_coding_system` case two rounds ago).

**Evidence ② - the positive control (P-86 ⑦).**  Injecting one deliberate error next to C 6774's
`dir = …` moves the else arm to `exit 1` with `injected_control` named once; reverted, the arm reads
`exit 0` and the file's md5 returns to the header's.

**Evidence ③ - the file's own gate (`extract-abi.py --check`): 43 -> 37 rows, diffed**: nine rows
retire (three `missing file-scope object` rows - `root_fd`, `root_fd_references`,
`saf_completion_sem` - and six `missing function` rows); five rows appear, **four of them the
`invented exported symbol` reading of this round's JNI entries** (the class `docs/ports/android.md`
432/462 records for `android.rs`, which no ported text can retire: the reference's symbol name is
pasted by `NATIVE_NAME`) and the fifth the whole-file line-count row, back at 9.8% because the port
now carries more lines than the reference.  Two `missing macro` rows retired **in this same commit**
by two rows in `docs/ports/excluded-macros.txt` (the two `#define android_saf_*_vnode
android_saf_tree_vnode` aliases are Rust `type` aliases, the `reg_char` precedent).

**Deviation (this slice's addition).**

33. **The `ftruncate` JNI entry stands in a module of its own.**  C 6558's pasted name is
    `ftruncate`, and the same file calls the *system* `ftruncate` at C 6563 - so a crate-root
    `pub unsafe extern "C" fn ftruncate` would be `E0428` against the mirror's declaration.  The
    port puts the definition in `mod androidvfs_jni_ftruncate` (the `lib/fcntl.rs` 213-222 device,
    whose comment names the same collision) and reaches the system entry as `crate::ftruncate`.

**Owed (this slice's additions).**

- `docs/baseline/port-lines-pending.tsv` (Owed 5): this file's row is stale and now reads over
  100% - **8,730** Rust lines against 7,950 C lines; the integrator's `--write-pending` re-run drops
  it.
- `tools/check-cond-eval.py src/androidvfs.c` still reads `14 conditions, 0 unknown, 5
  disagreements [NOT VERIFIED]` (slice 2's owed item).

**Not claimed (this slice).**  No runtime evidence, and none is possible here: no Android device, no
emulator, and this unit is not in the frozen configuration.  The semaphore was never posted or
waited on, no JNI entry was called from Java, no root directory was opened, and `special_vnodes`'s
second entry's `LISPSYM_INITIALLY` constant was never read.  What the round's evidence *is*: both
arms' `exit 0`, one control proving the harness reads this text, `--check` 43 -> 37 with the two
alias rows discharged in-commit, the measured layouts, and the ten gates.  **This file is not
finished**: C 6854-7950 remains (the next item is C 6869's `android_name_file`).
