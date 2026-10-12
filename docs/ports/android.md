# emacs/src/android.c (7,594 lines) — slices 1-2: C 1-1037 and C 1038-1869

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2597-2598 puts `android.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])`
> (2602), and `docs/baseline/frozen-build-sources.txt` lists no `android.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table** for these slices, the
> `## Evidence` readings and the `## Deviations`.  Port: `femacs/src/android.rs` (2,103 lines),
> both arms of every `#if` carried.
>
> **This file's ledger replaces the previous revision's conclusion** ("the file is not in the
> frozen build, so the port is the empty unit plus a note"), exactly as
> `docs/ports/androidselect.md`/`androidfont.md` did: P-86 ① rules that a three-line header
> comment is *not started*, not *done*.  The old note's measurements are still true and are still
> where the *file gate* comes from; what changed is the conclusion drawn from them.  **Slice 2
> (C 1038-1869) continues in the same unit**: it carries `android_run_debug_thread`,
> `android_user_full_name`, `android_is_special_directory`, the `#if 0` URL-encode pair,
> `android_get_home_directory`, `android_proc_name`, `android_create_lib_link`, the three
> `NATIVE_NAME` entry points (`getFingerprint`, `setEmacsParams`, `getProcName`),
> `android_init_emacs_service` and `android_init_emacs_pixmap`, so 1,869 C lines of the
> 7,594-line unit were carried by slice 2 (in a 2,103-line port).  **Slice 3 (C 1870-1992,
> 2026-10-12) continues in the same unit**: it carries the first three class tables -
> `android_init_graphics_point`, `android_init_emacs_drawable` and `android_init_emacs_window`
> (C 1930-1992's twenty-four `FIND_METHOD` calls) - so **1,992 C lines are carried in total** (in a
> 2,257-line port, md5 `90f0d173d3405e3d759a3dc1316a0fe0`).  **Slice 4 (C 1994-2084,
> 2026-10-12) continues in the same unit**: it carries `android_init_emacs_cursor`,
> `android_init_key_character_map` (the one that fetches a **static** method) and
> `android_init_emacs_handle` (one `FIND_METHOD` call plus a direct `GetFieldID`), so **2,084 C lines
> are carried in total** (in a 2,364-line port, md5 `d45d03d43f27071044051616a0f356f0`).  **Slice 5
> (C 2194-2253) added the six small shutdown/low-memory entries** (in a 2,464-line port), **except
> the 107-line hole C 2086-2192** (`initEmacs`'s `alloca`, Owed 11).  **Slice 6 (C 2255-2356,
> 2026-10-12)** now carries the five uniform event posters `sendConfigureNotify`, `sendKeyPress`,
> `sendKeyRelease`, `sendFocusIn` and `sendFocusOut`, so **2,356 C lines are carried** (in a
> 2,586-line port, md5 `4fa71db2a4a2b8069e16efc2ef01b2d2`) **except that one hole**.  **Slice 7 (C 2358-2548, 2026-10-12)** now carries the nine event posters (`sendWindowAction`,
> `sendEnterNotify`, `sendLeaveNotify`, `sendMotionNotify`, `sendButtonPress`, `sendButtonRelease`,
> `sendTouchDown`, `sendTouchUp` and `sendTouchMove`), so **2,548 C lines are carried** (in a
> 2,805-line port, md5 `bc92b9af3301974e08e0fd81c8fa0036`) **except that one hole**.  **Slice 8 (C 2550-2664, 2026-10-12)** now carries the six remaining simple posters
> (`sendWheel`, `sendIconified`, `sendDeiconified`, `sendContextMenu`, `sendExpose` and
> `sendDndDrag`), so **2,664 C lines are carried** (in a 2,937-line port, md5
> `bee6b42baa2e49f25a367a87d3d9531b`) **except that one hole**.  **Slice 9 (C 2666-2748, 2026-10-12)** now carries the two JNI string entries
> (`sendDndUri` and `sendDndText` - `GetStringLength`, `malloc`, `GetStringChars`, the two
> `return 0;` out-of-memory arms, `memcpy` and `ReleaseStringChars`), so **2,748 C lines are
> carried** (in a 3,031-line port, md5 `8ceee47bd50c34840acff69c85dead30`) **except that one
> hole**.  **Slice 10 (C 2750-2888, 2026-10-12)** now carries the two notification entries
> (`sendNotificationDeleted` and `sendNotificationAction` - the `GetStringUTFChars`/`strdup`/
> `ReleaseStringUTFChars` prologue both share, then `GetStringLength`/`malloc`/`GetStringChars`/
> `memcpy`/`ReleaseStringChars` and the `xfree` arm), `sendConfigurationChanged` (C 2844's
> `switch` becomes the port's `match`), and the three one-line answers
> (`shouldForwardMultimediaButtons`, `getQuitKeycode` - which has no prologue in C - and
> `shouldForwardCtrlSpace`), so **2,888 C lines are carried** (in a 3,193-line port, md5
> `eac24a21a96111d4bc4898ebffde02b1`) **except that one hole**.  **The next live definition is
> C 2891's `NATIVE_NAME (blitRect)`**, the first of the two `AndroidBitmapInfo` blitters; the
> table's last row is the remainder (C 2889-7594) and slices 3-10's own readings are in the
> dated sections at the end of this file.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597-2598's `ANDROID_OBJ` - both Android branches, unlike `androidselect.o`, which 2648 adds only in the `REALLY_ANDROID` one) | `feature = "have_android"` (on every item and on C 22-55's include graph) | an Android configuration |
| C 61 `#ifndef ANDROID_STUBIFY` … C 7518 (real JNI arm) | `all (feature = "have_android", feature = "not_android_stubify")` | Android, API calls real |
| C 7519 `#else` … C 7594 `#endif` (the `ANDROID_STUBIFY` arm) | `all (feature = "have_android", not (feature = "not_android_stubify"))` | Android, API calls stubbed - **in the remainder, not this slice** |
| C 70 `#ifdef __aarch64__` … C 72 (the `<arm_neon.h>` include) | `feature = "aarch64"` (text-only here; the four NEON regions are C 4064-4431) | an aarch64 Android configuration |
| C 176 `#ifdef THREADS_ENABLED` … C 181 (`android_jvm`) | `feature = "threads_enabled"` | threads on |
| C 241 `#ifdef __i386__` … C 246 (`unused_pointer`) | `feature = "i386"` | an x86 Android configuration |
| C 297 `#if __ANDROID_API__ < 21` … C 300 / C 304 (`select_pipe` vs `cancel_signal_received`) | `feature = "android_api_lt_21"` (the androidselect.rs `android_api_le_11`/`le_30` shape) | an Android configuration whose NDK min SDK is < 21 |
| C 310 `#if __ANDROID_API__ >= 21` … C 312 (`SELECT_SIGNAL`) | `not (feature = "android_api_lt_21")` | min SDK >= 21 |
| C 325 `#ifndef THREADS_ENABLED` … C 329 (`event_queue.thread`) | `not (feature = "threads_enabled")` | threads off |
| C 367/C 426/C 509/C 522/C 604/C 923/C 973 (the other API-level pairs) | the same `android_api_lt_21` | as above |
| C 401/C 614/C 685/C 752/C 930/C 1030/C 1495 (the other threads pairs) | the same `threads_enabled` | as above |
| C 1083 `#ifdef HAVE_STRUCT_PASSWD_PW_GECOS` … C 1090 (slice 2) | **no feature**: a frozen-configuration macro, so C 1084-1086 is the live arm and C 1089's `return "Android user";` is dead here (P-51's live-arm shape, the dired.rs `dirent_type` reading) | the frozen configuration has it 1 (`build/emacs-ref/src/config.h` 1510) |
| C 1125 `#if 0` … C 1360 `#endif /* 0 */` (slice 2, the `android_url_encode`/`android_get_content_name` pair) | **no feature**: dead in every configuration, so the port records the region instead of carrying it (the character.rs 91-97 / cm.rs 54-100 precedent) | never |
| C 1453 `#ifdef __clang__` … C 1459 (slice 2, the two `#pragma` arms) | **no feature**: a diagnostic pragma has no Rust counterpart and the port emits none either way (the process.rs 340 / timefns.rs 79-81 precedent) | never |
| C 1495 `#ifdef THREADS_ENABLED` … C 1499 (slice 2, `GetJavaVM`) | `feature = "threads_enabled"`; C's `#ifdef` has **no `#else` arm** (the `#endif` closes it directly), so there is no second arm to select | threads on |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428); `android_api_lt_21` is the one **derived** feature this slice adds and it
is not in `femacs/xtask/Cargo.toml` yet (owed, below).

`HAVE_WINDOW_SYSTEM` (configure.ac 7518) has no `#if` in this slice and therefore no gate.
`frame.rs`'s flag block 3.408 laid down is gated on `have_window_system`, and this unit carries
`have_android` **without** it, so that block does not apply here: nothing in C 1-1037 names a
`FRAME_*` flag, `struct frame` or `struct window` (android.c's include list at C 22-55 has
`lisp.h`, `blockinput.h`, `coding.h`, `systime.h`, `android.h`, `androidgui.h` - no `frame.h`,
no `window.h`).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-55 | the file's own include list (`allocator.h`, `assert.h` … `unistd.h`, `sys/*.h`, `minmax.h`, `android.h`, `androidgui.h`, `lisp.h`, `blockinput.h`, `coding.h`, `epaths.h`, `systime.h`) | file gate |
| 56-59 | `android_init_gui` and its comment | file gate (C's `#ifndef` opens at 61, below) |
| 60-72 | `#ifndef ANDROID_STUBIFY`, `<android/bitmap.h>`, `<android/log.h>`, `<linux/unistd.h>`, `<sys/syscall.h>`, the `__aarch64__`/`<arm_neon.h>` pair | real arm / `aarch64` |
| 73-140 | the seven class structs (`android_emacs_pixmap` … `android_emacs_handle`, C 74-139) | real arm |
| 141-254 | the file-scope objects (`android_api_level`, the `*_dir`/`*_path` names, the densities, `android_java_env`, the `emacs_gc_*` fields, `emacs_service`, `service_class`, the seven class records, `emacs_installation_time`, `event_serial`, `unused_pointer`, `signal_mask_changed_p`, `startup_signal_mask`), C 176-181 `THREADS_ENABLED`, C 241-246 `__i386__` | real arm / `threads_enabled` / `i386` |
| 255-271 | the event-handling note, `android_event_container` (C 264-271) | real arm |
| 272-308 | `android_thread_event_queue` (C 275-308; C 297-304 the API-level pair: `select_pipe` / `cancel_signal_received`) | real arm / `android_api_lt_21` |
| 309-312 | `#if __ANDROID_API__ >= 21` / `#define SELECT_SIGNAL SIGUSR1` | real arm / `not android_api_lt_21` |
| 313-333 | `android_event_queue` (C 314-333; C 325-329 the threads pair: the `thread` member) | real arm / `threads_enabled` |
| 334-339 | `event_queue`, the `android_run_select_thread` forward declaration | real arm (the forward declaration has no Rust item) |
| 340-399 | `android_init_thread_events` (C 346-399; C 367-384 the `< 21` pipe pair) | real arm / `android_api_lt_21` |
| 400-436 | `#ifdef THREADS_ENABLED`, `android_finalize_thread_events` (C 408-431), the `poll_thread` keys (C 434), C 426-429 the `< 21` pair | real arm / `threads_enabled` / `android_api_lt_21` |
| 437-459 | `android_get_poll_thread` (C 442-459; C 445-458 the threads pair) | real arm / `threads_enabled` |
| 460-499 | `android_set_task_name` (C 467-499; C 491-492 the `goto failure`, C 497-499 the `failure:` tail) | real arm |
| 500-670 | `android_run_select_thread` (C 501-670; C 509-516 the locals pair, C 522-603 the `< 21` arm, C 604-667 the `>= 21` arm) | real arm / `android_api_lt_21` / `threads_enabled` |
| 671-693 | `#if __ANDROID_API__ >= 21`, `android_handle_poll_signal` (C 674-691; C 685-689 the threads pair) | real arm / `not android_api_lt_21` / `threads_enabled` |
| 694-700 | `android_query_sem` (C 697), `main_thread_id` (C 700) | real arm |
| 701-764 | `android_init_events` (C 710-764; C 713-715 and 738-751 the API pair, C 752-763 the threads pair) | real arm / `android_api_lt_21` / `threads_enabled` |
| 765-776 | `android_pending` (C 766-776) | real arm |
| 777-801 | `android_wait_event` (C 782-801) | real arm |
| 802-832 | `android_next_event` (C 803-832) | real arm |
| 833-866 | `android_check_if_event` (C 834-866) | real arm |
| 867-909 | `android_write_event` (C 868-909) | real arm |
| 910-916 | the `android_urgent_query` note and object | real arm |
| 917-1037 | `android_select` (C 918-1037; C 923-925 the `static char byte` pair, C 930-932 the threads pair, C 973-982 the API pair, C 1030-1034 the threads pair) | real arm / `android_api_lt_21` / `threads_enabled` |
| 1038-1040 | the blank, C 1039's form feed and the blank that separate the two slices | real arm (slice 2) |
| 1041-1073 | `android_run_debug_thread` (C 1041-1073; C 1535 passes its address to `pthread_create`) | real arm |
| 1074-1079 | the blank, C 1075's form feed, the blank, the comment `/* Intercept USER_FULL_NAME … */` and the blank | real arm |
| 1080-1091 | `android_user_full_name` (C 1080-1091; C 1083-1090 the `HAVE_STRUCT_PASSWD_PW_GECOS` pair) | real arm / live arm only (P-51) |
| 1092-1098 | the blank, C 1093's form feed, the blank, the three-line comment and the blank | real arm |
| 1099-1123 | `android_is_special_directory` (C 1099-1123; C 1114-1119 the `switch (*name)` with two labels and no `default`) | real arm |
| 1124-1360 | the blank and C 1125-1360's `#if 0` … `#endif /* 0 */` around `android_url_encode` (C 1133-1225) and `android_get_content_name` (C 1232-1358) | dead in every configuration: recorded, not carried |
| 1361-1364 | the blank, the two-line comment `/* Return the current user's ``home'' directory … */` and the blank | real arm |
| 1365-1369 | `android_get_home_directory` (C 1365-1369) | real arm |
| 1370-1374 | the blank, the three-line comment `/* Return the name of the file behind a file descriptor FD … */` and the blank | real arm |
| 1375-1389 | `android_proc_name` (C 1375-1389; C 1378 the `char format[sizeof "/proc/self/fd/" + INT_STRLEN_BOUND (int)]`, C 1380-1384 the function-local `static struct allocator`) | real arm |
| 1390-1403 | the blank and the thirteen-line comment `/* Try to guarantee the existence of the `lib' directory … */` | real arm |
| 1404-1447 | `android_create_lib_link` (C 1404-1447; C 1413/1414/1426 the `goto failure`/`goto success` pair and C 1443/1445 the `success:`/`failure:` labels) | real arm |
| 1448-1460 | the blank, C 1449's form feed, the blank, the comment `/* JNI functions called by Java.  */`, the blank and C 1453-1459's `#ifdef __clang__` / two `#pragma` arms / `#endif` pair | real arm / pragma pair: recorded, not carried |
| 1461-1474 | `NATIVE_NAME (getFingerprint)` (C 1461-1473) and the blank | real arm |
| 1475-1686 | `NATIVE_NAME (setEmacsParams)` (C 1475-1686; C 1495-1499 the `THREADS_ENABLED` pair, C 1511-1513 the `PACKAGE_STRING`/`PACKAGE_BUGREPORT` format, C 1528-1537 the `pipe2`/`dup2`/`pthread_create` block, C 1608-1609 the `stat`/`get_stat_mtime` pair) | real arm / `threads_enabled` |
| 1687-1714 | the blank and `NATIVE_NAME (getProcName)` (C 1688-1714; C 1706's `goto finish`, C 1711's `finish:` label) | real arm |
| 1715-1717 | the blank, the comment `/* Initialize service_class, aborting if something goes wrong.  */` and the blank | real arm |
| 1718-1837 | `android_init_emacs_service` (C 1718-1837; C 1737-1742 the function-local `#define FIND_METHOD`, 41 calls, and C 1836 its `#undef`) | real arm |
| 1838 | the blank | real arm |
| 1839-1868 | `android_init_emacs_pixmap` (C 1839-1868; C 1858-1863 the function-local `#define FIND_METHOD`, one call, and C 1867 its `#undef`) | real arm |
| 1869 | the blank before the next live definition | real arm |
| 1870-1898 | `android_init_graphics_point` (C 1889-1894 the function-local `#define FIND_METHOD`, one call, and C 1897 its `#undef`) | real arm (slice 3) |
| 1899 | the blank | real arm (slice 3) |
| 1900-1928 | `android_init_emacs_drawable` (C 1919-1924 the same device, one call, C 1927 its `#undef`) | real arm (slice 3) |
| 1929 | the blank | real arm (slice 3) |
| 1930-1992 | `android_init_emacs_window` (C 1949-1954 the same device, **twenty-four** calls, C 1991 its `#undef`) | real arm (slice 3) |
| 1993 | the blank | real arm (slice 3) |
| 1994-2022 | `android_init_emacs_cursor` (C 2013-2018 the function-local `#define FIND_METHOD`, one call, C 2021 its `#undef`) | real arm (slice 4) |
| 2023 | the blank | real arm (slice 4) |
| 2024-2048 | `android_init_key_character_map` (C 2043-2047 fetches a **static** method, so no `FIND_METHOD` macro) | real arm (slice 4) |
| 2049 | the blank | real arm (slice 4) |
| 2050-2084 | `android_init_emacs_handle` (C 2069-2074 the same device, one call, C 2077 its `#undef`, C 2079-2083 a direct `GetFieldID`) | real arm (slice 4) |
| 2085 | the blank | real arm (slice 4) |
| 2086-2192 | `NATIVE_NAME (initEmacs)` - **blocked, not ported**: it calls bare `alloca` (C 2108/2121) and PORTING.md §3.2 forbids porting such a function before P-05's spike picks the device (Owed 11) | **blocked** (slice 5 records it here rather than writing it) |
| 2193 | the blank | real arm (slice 5) |
| 2194-2200 | `NATIVE_NAME (emacsAbort)` | real arm (slice 5) |
| 2201 | the blank | real arm (slice 5) |
| 2202-2214 | `NATIVE_NAME (quit)` | real arm (slice 5) |
| 2216-2225 | C 2216-2217's comment and `android_shut_down_emacs` | real arm (slice 5) |
| 2227-2233 | `NATIVE_NAME (shutDownEmacs)` | real arm (slice 5) |
| 2235-2245 | C 2235-2238's comment and `android_on_low_memory` | real arm (slice 5) |
| 2247-2253 | `NATIVE_NAME (onLowMemory)` | real arm (slice 5) |
| 2255-2276 | `NATIVE_NAME (sendConfigureNotify)` | real arm (slice 6) |
| 2277 | the blank | real arm (slice 6) |
| 2278-2299 | `NATIVE_NAME (sendKeyPress)` | real arm (slice 6) |
| 2300 | the blank | real arm (slice 6) |
| 2301-2322 | `NATIVE_NAME (sendKeyRelease)` | real arm (slice 6) |
| 2323 | the blank | real arm (slice 6) |
| 2324-2339 | `NATIVE_NAME (sendFocusIn)` | real arm (slice 6) |
| 2340 | the blank | real arm (slice 6) |
| 2341-2356 | `NATIVE_NAME (sendFocusOut)` (C 2355's `return ++event_serial;`) | real arm (slice 6) |
| 2358-2373 | `NATIVE_NAME (sendWindowAction)` | real arm (slice 7) |
| 2374 | the blank | real arm (slice 7) |
| 2375-2393 | `NATIVE_NAME (sendEnterNotify)` | real arm (slice 7) |
| 2394 | the blank | real arm (slice 7) |
| 2395-2413 | `NATIVE_NAME (sendLeaveNotify)` | real arm (slice 7) |
| 2414 | the blank | real arm (slice 7) |
| 2415-2433 | `NATIVE_NAME (sendMotionNotify)` | real arm (slice 7) |
| 2434 | the blank | real arm (slice 7) |
| 2435-2456 | `NATIVE_NAME (sendButtonPress)` | real arm (slice 7) |
| 2457 | the blank | real arm (slice 7) |
| 2458-2479 | `NATIVE_NAME (sendButtonRelease)` | real arm (slice 7) |
| 2480 | the blank | real arm (slice 7) |
| 2481-2502 | `NATIVE_NAME (sendTouchDown)` | real arm (slice 7) |
| 2503 | the blank | real arm (slice 7) |
| 2504-2525 | `NATIVE_NAME (sendTouchUp)` | real arm (slice 7) |
| 2526 | the blank | real arm (slice 7) |
| 2527-2548 | `NATIVE_NAME (sendTouchMove)` | real arm (slice 7) |
| 2550-2572 | `NATIVE_NAME (sendWheel)` (C 2567-2568's `float`-into-`double` pair) | real arm (slice 8) |
| 2573 | the blank | real arm (slice 8) |
| 2574-2588 | `NATIVE_NAME (sendIconified)` | real arm (slice 8) |
| 2589 | the blank | real arm (slice 8) |
| 2590-2604 | `NATIVE_NAME (sendDeiconified)` | real arm (slice 8) |
| 2605 | the blank | real arm (slice 8) |
| 2606-2623 | `NATIVE_NAME (sendContextMenu)` | real arm (slice 8) |
| 2624 | the blank | real arm (slice 8) |
| 2625-2644 | `NATIVE_NAME (sendExpose)` | real arm (slice 8) |
| 2645 | the blank | real arm (slice 8) |
| 2646-2664 | `NATIVE_NAME (sendDndDrag)` | real arm (slice 8) |
| 2666-2706 | `NATIVE_NAME (sendDndUri)` (the `GetStringLength`/`malloc`/`GetStringChars`/`memcpy`/`ReleaseStringChars` chain and its two `return 0;` arms) | real arm (slice 9) |
| 2707 | the blank | real arm (slice 9) |
| 2708-2748 | `NATIVE_NAME (sendDndText)` (the same chain, `ANDROID_DND_TEXT_EVENT`) | real arm (slice 9) |
| 2750-2779 | `NATIVE_NAME (sendNotificationDeleted)` (the `strdup`/`ReleaseStringUTFChars` pair and its two `return 0;` arms) | real arm (slice 10) |
| 2780 | the blank | real arm (slice 10) |
| 2781-2827 | `NATIVE_NAME (sendNotificationAction)` (the same `tag` prologue, then the `action` string's `malloc`/`memcpy` chain and the `xfree` arm) | real arm (slice 10) |
| 2828 | the blank | real arm (slice 10) |
| 2829-2862 | `NATIVE_NAME (sendConfigurationChanged)` (C 2844's `switch (detail)` with C 2846/2852's labels and C 2856's `default`) | real arm (slice 10) |
| 2863 | the blank | real arm (slice 10) |
| 2864-2873 | `NATIVE_NAME (shouldForwardMultimediaButtons)` | real arm (slice 10) |
| 2874 | the blank | real arm (slice 10) |
| 2875-2880 | `NATIVE_NAME (getQuitKeycode)` (no `JNI_STACK_ALIGNMENT_PROLOGUE` - C has none here) | real arm (slice 10) |
| 2881 | the blank | real arm (slice 10) |
| 2882-2888 | `NATIVE_NAME (shouldForwardCtrlSpace)` | real arm (slice 10) |
| 2889-7594 | **the remainder** - from C 2889's blank and C 2890's `JNIEXPORT void JNICALL` (C 2891's `NATIVE_NAME (blitRect)`, **the next live definition**) through C 7594's `#endif`: the two bitmap blitters, the rest of the JNI entry points, the SAFE_ALLOCA function at C 6020-6070 (blocked on the same P-05 decision), the non-JNI helpers, and C 7519's `#else` stub arm | not this slice |

The fifty rows tile C 1-1869 **contiguously** (blank lines included, two of them one-line rows for
the blanks at C 1838 and C 1869) rather than skipping the blanks between blocks, and the last row
runs on to C 7594, the reference's last line: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/android.rs` (this file's sibling header's mirror) to this same `android.md`
through its bare-stem fallback, so a table with gaps at blanks leaves 7 of `src/android.h`'s
non-blank lines uncovered and fails.  (`docs/ports/w32term.md` is the same shape and passes for
the same reason: its rows reach past `src/w32term.h`'s 929.)

## Evidence

The readings below are on the revision that carries **both slices** (C 1-1869,
`femacs/src/android.rs` 2,103 lines); the previous revision's readings for C 1-1037 alone are
superseded and are noted where they differ.

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/android2-lane/cargo cargo run -q -p xtask -- build-tu src/android.c`
→ `built src/android.rs -> ../build/femacs/android.o`, **exit 0**, and `build/femacs/android.o` is
**336 bytes** - an empty object, the same reading `docs/ports/androidmenu.md`/`androidselect.md`/
`androidfont.md` record for their units, and the same reading the previous revision produced.
Read **four** times (once per revision state of the file, all four `exit 0` and all four 336
bytes - see the budget paragraph below).  That is the expected reading and not a
proof of the new text: every item sits behind the file gate, so rustc compiles only
`include/config.rs` and the `#![allow]` attribute under the frozen feature set.

**② The flip of the arm (P-86 ③⑴/⑥/⑦).**  The two-`--cfg` command the class uses
(`docs/ports/androidmenu.md` §②):

```
CARGO_TARGET_DIR=/tmp/android2-lane/cargo cargo run -q -p xtask -- probe src/android.rs \
  /tmp/android2-lane/probe7.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'
```

* **②a, exit 1, one error line** (`error: aborting due to 1 previous error; 2 warnings emitted`):

  ```
  error[E0609]: no field `sa_sigaction` on type `sigaction`
     --> .../build/probes/android/_root.rs:1010:8
  1010 |     sa.sa_sigaction = Some (android_handle_poll_signal);
  help: a field with a similar name exists
  ```

  **Classification by file: 1 error, at C 749's line (slice 1) and 0 errors in any slice-2 line;
  0 errors in the lines of any included mirror.  By cause: the one error is the *mirror gap* the
  previous slice already reported** - the included `lib/include/signal.rs` copy carries the frozen
  Darwin `struct sigaction` (`__sigaction_u` + `sa_mask` + `sa_flags`), which has no direct
  `sa_sigaction` member, and C 749's member belongs to bionic's (`sizeof` 32, measured in the
  previous revision).  No object was produced.  This is the `docs/ports/androidfont.md` ②c shape,
  and per P-86 ⑦ the criterion is not "0 errors in the unit" - which is why ②b exists.
* **②b, all sub-gates set (`+ android_api_lt_21`), exit 0, 11 warnings, object 17,376 bytes**:

  ```
  CARGO_TARGET_DIR=/tmp/android2-lane/cargo cargo run -q -p xtask -- probe src/android.rs \
    /tmp/android2-lane/probe8.o --crate-type lib --emit obj -C panic=abort \
    --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
    --cfg 'feature="android_api_lt_21"'
  → exit 0, 0 errors, 11 warnings, `-rw-r--r--  17376  probe8.o`
  ```

  With the `__ANDROID_API__ < 21` arm selected, no line of the unit assigns `sa.sa_sigaction`, and
  the **pristine** graph - no harness - compiles clean and emits an object.  The warning inventory
  is byte-for-byte the previous revision's (7 FFI-safety notices from the mirrors' opaque types -
  `AndroidBitmapInfo`, 2 `_jmethodID`, 4 `android_vdir` - 2 unused doc comments and 1 each for the
  `read`/`write` runtime symbols `unistd.in.rs` declares): **slice 2 adds no warning and no
  error.**  The object grew from the previous revision's 5,232 bytes (C 1-1037) to 17,376.
* **An arm-compile reading that caught a real defect in this slice (the P-86 ⑦ mutation class,
  measured by accident and kept as evidence that ② is not an empty reading).**  The first
  `probe` run after slice 2's text landed (the first of the eight above) read
  `error[E0609]: no field `constructor_mutable` on type `android_emacs_service` ... _root.rs:...`
  in addition to ②a's error: the generated `FIND_METHOD!` call list had been spliced across the
  boundary between `android_init_emacs_service` and `android_init_emacs_pixmap`, which deleted the
  second function's prologue and its `macro_rules! FIND_METHOD` (whose body names
  `pixmap_class`).  The reading named the defect, and the fix is what the final revisions of ②a and
  ②b above read.  It is not a mutation test of the ported text; no injected-mutant reading was
  taken for this slice.
* **Budget, reported honestly: this lane overran both limits.**  Five `build-tu` invocations (one
  rejected by cargo before any compile - wrong working directory - and four compiles, **all
  `exit 0`, all 336 bytes**) and **eight** `probe` runs, against a limit of three each.  The eight
  probes were: ①a 42-error reading of a revision whose generated `FIND_METHOD!` call list had been
  spliced across the two `android_init_*` functions (it named the defect below); ② a 2-error
  reading of the same revision; ③ the first 1-error reading (the pre-`getFingerprint` revision);
  ④ the first `+ android_api_lt_21` object (16,984 bytes); ⑤ the same config after
  `NATIVE_NAME (getFingerprint)` was added (17,376 bytes); ⑥ a base-config reading before the
  comment-only edits; ⑦ and ⑧ the two final-revision readings quoted above.  The repeats changed no
  reading.  The readings quoted in ②a/②b are ⑦ and ⑧, i.e. the **final revision**; the two edits
  between ⑧ and this ledger are comment-only (the port's C-line references and the unit's header).

**③ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/android.c femacs/src/android.rs`
(exit 1 in both revisions; the failure is the file's own boundary, see below):

| | before (C 1-1037 only) | after (C 1-1869) |
| --- | --- | --- |
| `- ` problem lines | 135 | **131** |
| `- missing function …` for the 8 slice-2 definitions | 8 | **0** |
| `- missing macro FIND_METHOD` | 1 | **0** |
| invented-exported-symbol lines | 0 | **3** |
| does-not-mention lines | 2 | **4** |
| line drift | `82.7%: C 7594 vs Rust 1311` | `72.3%: C 7594 vs Rust 2103` |

The nine lines that disappeared are exactly this slice's eight functions
(`android_run_debug_thread`, `android_user_full_name`, `android_is_special_directory`,
`android_get_home_directory`, `android_proc_name`, `android_create_lib_link`,
`android_init_emacs_service`, `android_init_emacs_pixmap`) and its macro `FIND_METHOD`; the
`- missing …` lines that remain are C 1870-7594, this slice's boundary - not defects.  The five
lines that appeared are quoted and classified in `## Deviations` (the `switch`, the `goto` and the
`NATIVE_NAME` bullets) and in `## Owed` 8-10; none of them is a port defect.  The previous revision's two `does-not-mention` rows (slice 1's
`android_set_task_name` `goto` and `android_write_event` `break,case,default,switch`) still print;
this slice adds `android_create_lib_link` (`goto`) and `android_is_special_directory`
(`case`, `switch`).  The drift is the whole-file ratio against a partially-ported file; this slice's
own ratio is C 829 lines → the port's 792 new lines (-4.5% on C's count for the same text, against
+26.4%
for slice 1).

**④ The other gates (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  → info docs/ports/android.md: 50 range row(s) cover src/android.c (7594 lines; 0 blank
    line(s) between blocks need no row)
    info docs/ports/android.md: 50 range row(s) cover src/android.h (363 lines; …)
    … 18 ledger(s) bound (a C range table), 0 failure(s)          [exit 0]
python3 tools/check-declarations.py
  → FAIL femacs/src/android.rs: literal x1: macro_rules! PACKAGE_STRING { () => { "…" }; }
    (the `…` is `<config.h>` 2205's `PACKAGE_STRING`, the product name and version, elided here
    for the reason this section gives at the end)
    14 declaration sites (0 forbidden, 14 allowed literals), 1 failures   [exit 1]
python3 tools/check-port-lines.py
  → stale src/xfns.c: no longer under the threshold
    84 file(s) under ten percent, 177,325 reference line(s); 85 ratchet row(s), 0 new, 1 stale
                                                                       [exit 0; no android row]
```

`check-declarations`' single failure is **this slice's one unavoidable finding, and the row it
needs is outside this lane's writable set**: C 1511-1513's log format is C's
`"Initializing "PACKAGE_STRING … "`, and `<config.h>` 2205 defines `PACKAGE_STRING` as the product
name and version, so the ported unit carries that name inside a string literal - the class
`docs/baseline/declarations-allowlist.tsv` exists for (its six existing rows are all "C's own
literal, printed by a banner").  The name is elided in this ledger too, because a ledger is scanned
for it as well (the `title` class).  Carrying a *different* string, or splitting the product name
across two literals so the scanner cannot match it, would be an unfaithful port and a gate dodge
respectively, so the text stands and the row is owed (item 10 below).

**⑤ Not measured here.**  No physical Android device, no NDK, and no runtime: the arm readings are
compile-level (②a/②b), the frozen reading is an empty object (①), and **nothing in this slice was
executed** - no thread, no JNI call, no `pipe2`, no `stat`, no `getline`, no `FIND_METHOD`
lookup.  The `sigaction` mirror gap still blocks the pristine base-config arm (②a), so the object
readings exist only with `android_api_lt_21` selected.


## Deviations

* **`<pthread.h>`, `<semaphore.h>` and six `<signal.h>` names have no mirror** and are declared
  in the unit (P-88/P-70; the `sysdep.rs` `<pthread.h>` block and androidselect.rs's `<dlfcn.h>`
  block are the precedents): `pthread_mutex_t`/`pthread_cond_t`/`pthread_t`/`pthread_key_t`,
  `sem_t`, the thirteen `pthread_*`/`sem_*` entries, `gettid`, `pipe2`, `kill`, `sigfillset`,
  `sigdelset`, `sigwait`, `sig_atomic_t`, `siginfo_t` (opaque - only a pointer here), `SA_SIGINFO`.
  Their values are the bionic ones measured in ⑦.  Each should move into a mirror once one exists.
* **The frozen system mirrors are the frozen target's, not Android's.**  `signal.rs`'s `sigset_t`
  (4 bytes) and `sigaction` (16 bytes, `__sigaction_u.__sa_sigaction`) are Darwin's where bionic's
  are 8 and 32 with `sa_sigaction` (⑦); `signal.rs`'s `SIGUSR1`/`SIGIO` are 30/23 where bionic's
  are 10/29; `sys_select.rs`'s `fd_set`/`FD_ZERO`/`FD_SET`/`FD_ISSET`/`FD_CLR` are Darwin's
  (`__darwin_check_fd_set_overflow`); `errno.rs`'s `errno ()` is Darwin's `*__error ()` where
  bionic's is `*__errno ()`.  The port uses them as they are; the Android ABI is **not** what
  compiles here, and ④'s one patch class is where the `sigaction` difference bites.  This is the
  mirror round owed below.
* **`sa.sa_sigaction = Some (android_handle_poll_signal)`** (C 749) keeps C's member name.  The
  frozen `signal.rs` has no such member, so this is the ②a error; with `android_api_lt_21` the arm
  never reaches it and the pristine graph compiles (②b/②c).
* **`volatile`**: C 303's `volatile sig_atomic_t cancel_signal_received` is a plain `sig_atomic_t`
  field (Rust has no volatile field form).
* **`goto` and labels (PORTING.md 2.2)**: C 491-492's `goto failure` and C 497-499's `failure:`
  become the labeled block `'failure:` whose `break 'failure` is the jump, with C's `failure:` tail
  one copy after it.  The `goto` token does not survive (⑤, owed).
* **`switch` (C 894-908)**: `match (*event).r#type { ANDROID_KEY_PRESS | ANDROID_WINDOW_ACTION =>
  … , _ => {} }`; C's two case labels are one `|` arm and `default:` is `_` (⑤, owed).  The union
  field is `r#type` (the C keyword `type`).
* **C's `malloc`/`free` (C 872, 902)**: android.c reaches `<stdlib.h>` transitively; the
  port includes `../lib/include/stdlib.rs` at C 37's place (the `androidterm.rs` shape).  C's
  `sizeof *queue`/`sizeof *container` are `std::mem::size_of::<android_thread_event_queue> ()` /
  `<android_event_container> ()`.
* **`__aarch64__` (C 70-72)**: `#include <arm_neon.h>` has no mirror and no name of it is used
  before C 4064, so this slice carries the pair as a comment; the P-88 declaration segment belongs
  to the slice that first names a NEON intrinsic.
* **`INT_STRLEN_BOUND` (C 28)**: the `intprops.rs` mirror is blind and no unit includes it; the
  unit defines the macro (the androidselect.rs 116-119 shape, over `size_of::<$t>`).
* **`sizeof "/proc/self/task//comm"` is 21**: the array length is written `INT_STRLEN_BOUND!
  (c_long) + 21`, with C's expression in the comment.
* **C's file-scope objects are tentative definitions** (PORTING.md 0.1-13/1.5), so the port spells
  every one out: `0` / `false` / `std::ptr::null_mut ()` / `std::mem::zeroed ()` (the seven
  `static struct` class records, `service_class`, `emacs_installation_time`,
  `startup_signal_mask`, `event_queue`, `android_query_sem`).
* **Locals C leaves uninitialised and the port zeroes** (PORTING.md 1.5): `fd`, `lwp`, `length`
  (C 472-474), `rc`/`data` (C 505-506), `nfds`/`readfds`/`byte` (C 510-512), `signals`/`waitset`/
  `sig` (C 514-515), `queue` (C 676), `sa` (C 714), `i` (C 769), `container` (C 806, 841),
  `nfds_return`/`nevents` (C 921).  C 924's function-local `static char byte` is a `#[cfg]`-gated
  `static mut`.
* **`MIN (16, length)` (C 487) is `MIN! (16usize, length)`**: C's `int`/`size_t` promotion is the
  `usize` literal.
* **C 339's forward declaration** has no Rust item (a function item is visible for the whole
  module); the definition is C 501-670.
* **The `#else` arm of C 61 (C 7519-7594, the `ANDROID_STUBIFY` arm)** is in the remainder, not
  this slice: this port carries the *real* arm of the whole-file `#ifndef`, and the stub arm is
  owed by the slice that ports it.
* **Line count**: C 1,037 (this slice) vs the port's 1,311 (+26.4%) - the per-item `#[cfg]`s, the
  in-unit declarations, Rust's brace-only lines and the C-line comments.

**Slice 2 (C 1038-1869).**

* **The declaration segment.**  Six names this slice uses have no reachable mirror, so they are
  declared in the unit at C's own include line or at the use site (P-88 ①, each with its C
  prototype in a `/// C: …` line): `fdopen` and `getline` (C 37's `<stdio.h>`:
  `lib/include/stdio.rs` is the frozen *system* header mirror and carries neither, while the gnulib
  *template* mirror `lib/include/stdio.in.rs` 238/248 does carry both but is reached by no unit -
  `rg -ln 'stdio.in.rs' femacs/src femacs/lib` is empty), `strdup` (C 37's `<string.h>`; the mirror
  has no declaration, `sysdep.rs` 727 / `emacsclient.rs` 302 declare it in their own units),
  `setenv` (C 37's chain `<stdlib.h>`; unreachable `stdlib.in.rs` 248, the `sysdep.rs` 956 shape),
  `dirname` (C 29's `<libgen.h>`: **no mirror exists anywhere in the tree** - this unit is the first
  of the port to name it), `hexbuf_digest` (C 51's `lisp.h` 4247; the `lisp.rs` mirror lacks it, the
  `emacs.rs` 192-193 shape).  Three more edges were added to the include graph at C's own lines:
  `careadlinkat.rs` (C 24), `fingerprint.rs` (C 27) and `stat-time.rs` (C 35) - the previous slice's
  comment said those headers "name nothing in C 1-1037", which is why they were absent.
* **`PACKAGE_STRING`/`PACKAGE_BUGREPORT`/`DIRECTORY_SEP`** (C's `<config.h>`).  `config.rs` carries
  only the generated configuration keys, so the unit carries them: the two report strings as
  `macro_rules!` bodies (C 1511-1513 puts them *between* string literals, and `concat!` expands a
  macro_rules call - the `dispnew.rs` 70/170 shape; `str` literals there, because `concat!` cannot
  join `c""` literals) and `DIRECTORY_SEP` as a `c_char` constant, because C 1117's `case '/'` needs
  a *pattern* and a Rust pattern cannot hold the `b'/' as c_char` cast (the `fileio.rs` 309-312
  shape).  The `PACKAGE_STRING` body is the declaration the `literal`-class finding above is about.
* **`switch` (C 1114-1119)**: `match (*name)` with the two labels as arms and `_ => {}` for C's
  fall-through out of the switch, so `switch`/`case` do not survive (owed row, item 8).
* **`goto` and labels (PORTING.md 2.2)**: `android_create_lib_link`'s `goto failure`/`goto success`
  (C 1413/1414/1426) become the nested `'failure:`/`'success:` labelled blocks, with C's `success:`
  tail (`close (fd)`) inside the outer block and its `failure:` tail (`return;`) after it; and
  `getProcName`'s `goto finish` (C 1706) becomes a `'finish:` block with C's `finish:` tail
  (`free (buffer); return array;`) after it.  The `goto` token does not survive either (owed row, item 9).
* **`NATIVE_NAME` (C 344, an excluded macro) and `JNIEXPORT`/`JNICALL`**: the three entry points are
  Rust fns named after the macro *argument* (`getFingerprint`, `setEmacsParams`, `getProcName`) and
  carry the pasted symbol as `#[unsafe(export_name = "Java_org_femacs_EmacsNative_<name>")]` - the
  prefix is **our own** package, not the reference's (`src/android.h` 344; status 3.503) -
  the replacement `docs/ports/excluded-macros.txt` 147 prescribes.  `JNIEXPORT`/`JNICALL` (jni.h's
  visibility and calling-convention attributes) have no mirror and no Rust text: the
  `pub unsafe extern "C"` signature is where they land.
* **`ANDROID_DELETE_LOCAL_REF`** (C 340-342): the unit defines `macro_rules! ANDROID_DELETE_LOCAL_REF`
  with C's own expansion, because the exclusion reason ("`JNINativeInterface` has no mirror") has
  been answered by `include/jni.rs` (the androidselect.rs 121-129 shape; P-86 ⑪).
* **Two `jfloat` parameters are assigned to `double` objects** (C 1506-1508): C's implicit float to
  double promotion is written `as f64`, and the `size_t`->`jsize` narrowing of `getProcName`'s
  `NewByteArray` argument (C 1704) is `as jsize` - Rust has no implicit numeric conversion
  (PORTING.md 0.3).
* **C 1564 and C 1578 test the wrong variable**: after setting `android_lib_dir` and
  `android_cache_dir`, the error checks read `android_files_dir` (and C 1548/1550's pattern is what
  they were copied from).  The port keeps C's text - the defect is the reference's, and the byte
  counts and behaviour of the failure path are identical either way; changing it would be a silent
  divergence (PORTING.md 0.3).
* **The function-local `static struct allocator allocator` (C 1380-1384)** keeps C's one name for
  both the type and the object: Rust's type and value namespaces keep them apart, so
  `static mut allocator: allocator = allocator { allocate: Some (malloc), … }` is the same
  declaration.  `careadlinkat`'s function-pointer argument is `Some (readlinkat)`.
* **C 1378's `char format[sizeof "/proc/self/fd/" + INT_STRLEN_BOUND (int)]`** is
  `[c_char; std::mem::size_of::<[c_char; 15]> () + INT_STRLEN_BOUND! (c_int)]` (`/proc/self/fd/` is
  14 bytes plus the terminator); the three `sizeof`s of C 1466-1470 are the `size_of`/`size_of_val`
  family `docs/ports/token-aliases.tsv` folds, so the occurrence check counts four for three (the
  array length is written twice).
* **`#if 0` (C 1125-1360) and the `#pragma` pair (C 1453-1459)** are recorded, not carried: dead in
  every configuration / no Rust counterpart (the character.rs 91-97, cm.rs 54-100, process.rs 340
  precedents).
* **Line count**: slice 2 is C 829 lines (C 1038-1869) against the port's 792 new lines (-4.5% on
  C's count for the same text: the `#if 0` region is a comment here and one `FIND_METHOD` call is one
  line); the whole file is now 2,103 lines against C 1,869 (+12.5%), against slice 1's +26.4%.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. ✅ **Done in 3.412** — `femacs/xtask/Cargo.toml`'s `[features]` table: **`android_api_lt_21`** (comment: the derived
   condition `__ANDROID_API__ < 21`, android.c 297/310/367/426/509/522/604/672/713/738/923/973;
   its negation is C's `>= 21` arm).  Without it none of this slice's `#if` pairs can be selected.
2. ✅ **Done** — `docs/ports/macro-body-notes.tsv`: `android_write_event` →
   `break,case,default,switch` landed with this slice; `android_set_task_name` → `goto` could not
   be measured until 3.414 fixed the extractor's `//`-in-string defect, and is now recorded there
   (C 481/486's `goto failure` and the `failure:` label at C 491).
3. `docs/ports/line-drift-tolerance.tsv`: one row for `android` once the last slice lands (the
   whole-file reading is now the 72.3% of ③; slice 2's own ratio is -4.5% on its own C lines).
4. ✅ **Settled by the tool's own run** — `tools/check-port-lines.py` now reads
   `85 ratchet row(s), 0 new, 1 stale` (the stale row is `src/xfns.c`, another lane's) and **no
   longer carries an `android` row**: C 1,869 carried against the unit's 7,594 lines is 24.6% of the
   reference, above the ten-percent threshold, so nothing needs a `--write-pending` rewrite here.
5. ✅ **Done in 3.414** — the tool was fixed, not the symptom:
   `tools/cinventory.py`'s `blank_comments` now models string and character literals, so C 470-471's
   `"/proc/self/task//comm"` no longer blanks the rest of the line.  The false positive
   (`does not mention comm, failed, proc, self, set, task`) is gone and the real finding - the
   `goto` of item 2 - took its place.  See `docs/status.md` 3.414 for the 634-file reading and the
   second manifestation it uncovered in `w32fns.c` 4127.
6. **The mirror round that blocks the pristine base-config arm: `lib/include/signal.rs` needs the
   bionic `sa_sigaction` member C 749 names** (or an Android `signal.h` mirror).  That file is
   another lane's in-flight mirror and the change needs `tools/check-all-units.sh` (P-87), which is
   why it is not touched here; until it lands the base config reads ②a and the unit's own body
   needs ④'s harness.
7. The in-unit declarations of slice 1's Deviations belong in mirrors once those exist (P-88 ⑤): the
   `<pthread.h>`/`<semaphore.h>` block, the `<signal.h>` extras, `gettid`/`kill` and the
   `<android/log.h>` pair.  **`pipe2` is settled and stays where it is** (3.414): bionic's
   `<unistd.h>` declares it for this configuration, and `docs/baseline/mirror-dupes.tsv` now
   carries the ratchet row that says why the mirror's Darwin redirect and this declaration are two
   configurations' arms rather than a copied declaration.
8. `docs/ports/macro-body-notes.tsv` (not in this lane's writable set): **two rows for slice 2**,
   both of the recorded-deviation class the file exists for -
   `android_is_special_directory` → `case,switch` (C 1114-1119's `switch (*name)` with its two
   labels and no `default` becomes a `match`, so the three tokens have no counterpart) and
   `android_create_lib_link` → `goto` (C 1413/1414/1426's `goto failure`/`goto success` become the
   `'failure:`/`'success:` labelled blocks, PORTING.md 2.2).  Both lines print in `--check` today;
   every other token of both bodies is present.  (`getProcName`'s `goto finish` needs no row: a
   `NATIVE_NAME` definition is not in `ci.functions`, so that body is compared by nothing - see 9.)
9. **`tools/cinventory.py`'s function scan is blind to `NATIVE_NAME` definitions, and the port pays
   for it in `extract-abi --check`.**  C 1461/1475/1688 define
   `JNIEXPORT … JNICALL NATIVE_NAME (getFingerprint/setEmacsParams/getProcName) (…)`; the C symbol
   each one emits is `Java_org_femacs_EmacsNative_<name>` (the excluded macro pastes it, with our
   own package as the prefix since status 3.503), and the
   port writes that symbol as `#[unsafe(export_name = …)]` on the Rust fn - the replacement
   `docs/ports/excluded-macros.txt` 147 prescribes.  But `ci.functions` lists neither the pasted
   C name nor the macro argument, so `c_exported` and `whole_reference("function")` cannot back the
   attribute and the rule at `tools/extract-abi.py` 2045-2051 reports **three lines that are not
   port defects**:
   `fn getFingerprint carries #[unsafe(no_mangle)] but the reference does not define a non-static
   function of that name (invented exported symbol)`, and the same for `setEmacsParams` and
   `getProcName`.  Owed: teach the scan that `NATIVE_NAME (x) (params)` is a non-static definition of
   `Java_org_femacs_EmacsNative_x` (then the three lines disappear and the attributes stay), or
   record the class as allowed.  It is the P-86 ⑩/⑪ family of "the gate has not been taught this
   shape", and the reference does export those three symbols.
10. `docs/baseline/declarations-allowlist.tsv` (not in this lane's writable set): **one `literal`
   row for the product name slice 2's log format carries**, of the same class as its six existing
   rows.  `tools/check-declarations.py` prints the exact site -
   `FAIL femacs/src/android.rs: literal x1: macro_rules! PACKAGE_STRING { () => { "…" }; }`
   (value elided, see Evidence ④) - and today's reading is `1 failures`.  The row's reason is the one that class
   uses: `<config.h>` 2205's `PACKAGE_STRING`, which C 1511-1513 concatenates into
   `__android_log_print`'s format, is ported semantics and cannot be spelled otherwise; the port
   keeps C's `macro_rules!` name so the body's two tokens survive (see Deviations).
11. **The mirror round slice 2's declaration segment owes** (P-88 ⑤/P-87): `fdopen` and `getline`
   belong in `lib/include/stdio.rs` (the frozen `<stdio.h>` mirror; the gnulib template mirror
   `lib/include/stdio.in.rs` has them at 238/248 but no unit reaches it), `strdup` in
   `lib/include/string.rs`, `setenv` in `lib/include/stdlib.rs`, `dirname` in a new `<libgen.h>`
   mirror (none exists in the tree), `hexbuf_digest` in `src/include/lisp.rs` (lisp.h 4247), and
   `PACKAGE_STRING`/`PACKAGE_BUGREPORT`/`DIRECTORY_SEP` in `src/include/config.rs` (the generator
   reads the generated `config.h` and drops the three).  Each is a *different* lane's file, and the
   moves need `tools/check-all-units.sh` (P-87): until they land, this unit's declarations are the
   only definitions in play.

11. **`NATIVE_NAME (initEmacs)` (C 2086-2192) and the SAFE_ALLOCA function (C 6020-6070) are
   blocked on P-05.**  PORTING.md §3.2 says the `alloca` device is not yet chosen (three candidates
   are to be compared in a spike with memory pressure, and **until one is chosen, files containing
   `alloca` must not be ported in bulk**), and both definitions use it - `initEmacs` at C 2108/2121
   (bare `alloca`, *inside a loop* at C 2121, which §3.2 says needs a per-point special case) and
   the other at C 6031/6064 (`USE_SAFE_ALLOCA` + `SAFE_ALLOCA`).  The whole rest of the file (5,340 C
   lines) is `alloca`-free, so this is a two-function blocker, not a file-wide one: the range table
   records both rows and the slices step over them (the `android_update_selection` shape of
   `docs/ports/androidterm.md`).  When the spike lands, the two rows are replaced by their slices'
   rows.

### Not claimed

Every line of all three slices is behind gates the frozen configuration leaves false: ① is an
empty object, the arm readings (②a/②b and ③ below) are compile-level, and **nothing here was
executed** - no thread,
no semaphore, no JNI call, no `Java_org_femacs_EmacsNative_*` lookup, no `pipe2`, no `stat`, no
`getline`, no event, no `select`.  Slices 2-10's C lines (C 1038-2084, C 2194-2253,
C 2255-2356, C 2358-2548, C 2550-2664, C 2666-2748 and C 2750-2888) are carried; the
**unported** part is C 2086-2192 (blocked on P-05's `alloca` decision) plus C 2889-7594, and
`extract-abi.py --check`'s remaining `- missing …` list (115 `missing function` rows, the two
file-scope objects and the thirty-five `invented exported symbol` rows of Owed 9) and the
58.0% drift are that boundary, not defects.  The three `invented exported symbol` lines are the extractor blind spot of Owed 9, the
one `check-declarations` failure is the allowlist row of Owed 10, and the two `does not mention`
lines are the recorded rows of Owed 8 - none of the six is a port defect.  The `android_api_lt_21`
configuration compiled to an object (②b, 17,376 bytes) and was not run; the pristine base-config arm
still stops on `lib/include/signal.rs`'s missing bionic `sa_sigaction` member (②a, Owed 6), so no
object exists for it.

## 2026-10-05: `PACKAGE_STRING` carries the port's own name (status 3.433)

The startup log line C 1511-1513 builds from this macro is the program's self-declaration, so the
macro body now spells the port's own name with the same version number; the frozen configuration's
measured value stays in `docs/baseline/config/` (PORTING.md 0.1-19's name half).  `PACKAGE_BUGREPORT`
is left as the measured value, which status 3.433 names as the boundary of this half.

## 2026-10-05: the JNI names carry the port's own package (3.503)

C 344's `NATIVE_NAME` pastes the *reference's* Java package into every exported symbol; that name is
what *this* app's class is found by, so the seventh half of PORTING.md 0.1-19 renames both sides at
once: the 40 Kotlin targets declare `package org.femacs` (11 identity strings with it:
`EmacsActivity`'s extra key, `EmacsDesktopNotification`'s notification tags and `RemoteViews`,
`EmacsDocumentsProvider`'s authority, `EmacsService`'s `package:` URI, `EmacsApplication`'s
`getApplicationInfo`, `EmacsNoninteractive`'s `loadClass`), and this unit's 41 `FindClass`/descriptor
lines plus the three `export_name`s move with them (`Java_org_femacs_EmacsNative_*`).  P-83 (1)(2)
now says it in one line: the package is ours, the class/field/method names are byte-identical on
both sides.  Evidence: `spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/*.kt` -> **exit
0** (the probe re-homes the reference `.java` stand-ins into the same package, so the whole batch is
still a real compile; measured 3.503, `JAVA_TOOL_OPTIONS=-Xmx2g`).  The Rust side has no compiler on
this machine for the `have_android` arm (the mirror blockers of P-86 (7)), so its evidence is textual
plus the descriptor-by-descriptor `grep`.  `PACKAGE_BUGREPORT` is untouched: the address is the
reference's and points upstream (the 3.433 boundary).

## 2026-10-05: `PACKAGE_BUGREPORT` is this repository's own (status 3.613)

The macro is what the Android side tells a user with a bug, so it now spells this repository's
issue page (PORTING.md 0.1-19's ninth half, which replaces the 3.433 boundary recorded above for
this one constant).  The Kotlin target's own message moved with it; the frozen configuration's
measured address stays in `docs/baseline/config/`.

## 2026-10-12: slice 3 (C 1870-1992) - the first three class tables

`android_init_graphics_point` (C 1870-1898), `android_init_emacs_drawable` (1900-1928) and
`android_init_emacs_window` (1930-1992).

### Evidence (this slice's readings)

**③ The arm flip of this slice.**  The same three-`--cfg` command ②b uses
(`have_android`, `not_android_stubify`, `android_api_lt_21`):

```
cd femacs && CARGO_TARGET_DIR=/tmp/android-lane cargo run -q -p xtask -- probe src/android.rs \
  /tmp/android-lane/arm.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
  --cfg 'feature="android_api_lt_21"'
```

→ **exit 0, 0 errors, 11 warnings emitted**, and this slice's text needed **one** probe run
(the limit for a slice is three): the three tables' `FIND_METHOD` macros, the `NewGlobalRef` pair
and the twenty-six method names all read correctly on the first pass.  The warnings are the unit's
existing set (`android_vdir`/`_jmethodID`/`AndroidBitmapInfo` FFI-safety notes and the two
`suspicious definition of the runtime read/write symbol` lines ②b already records).

**④ The checklist reading.**  `extract-abi.py --check` on this revision: **120 `missing function`**
rows (down from 123 - the three tables retire), **2 `missing file-scope object`**
(`android_servicing_query`, `android_query_context`), **3 `invented exported symbol`** (Owed 9's
class), **0 `missing macro`**, **0 `missing constant`**, **0 `does not mention`** and the
**70.3%** line-count row (C 7,594 against the port's 2,257, inside this unit's
`docs/ports/line-drift-tolerance.tsv` row).

### Deviations this slice adds

- **The three `FIND_METHOD` macros** are in-function `macro_rules!` declarations with the field
  name as `$c_name:ident` (the `android_init_emacs_pixmap` device of slice 2, C 1858-1863/1867);
  C's `#undef` is the macro's scope ending, and C's two-statement body (the assignment and the
  `eassert`) is the macro's two statements.
- **The `org/femacs/` rename** applies to this slice's two class names and three signature strings
  (`org/femacs/EmacsDrawable`, `org/femacs/EmacsWindow`, and the `Lorg/femacs/EmacsWindow;`/
  `Lorg/femacs/EmacsCursor;` parameters of C 1964/1973/1981), the 2026-10-05 section's rule.
- **C 1881-1883's `(jclass) NewGlobalRef (…, (jobject) old)`** needs no cast text in Rust: the
  mirror's `jclass` and `jobject` are the same `*mut c_void` (`include/jni.rs` 49-50), so the two
  C casts are identity conversions and only `old as jobject` is written.
- **C 1930-1992's twenty-four calls** keep C's order, including the `damage_rect` one whose comment
  (C 1982-1985) explains why the ID comes from `EmacsWindow`.

## 2026-10-12: slice 4 (C 1994-2084) - cursor, key character map and handle tables

`android_init_emacs_cursor` (C 1994-2022), `android_init_key_character_map` (2024-2048) and
`android_init_emacs_handle` (2050-2084).

### Evidence (this slice's readings)

**⑤ The arm flip of this slice.**  The same three-`--cfg` command ③ uses:

```
cd femacs && CARGO_TARGET_DIR=/tmp/android-lane cargo run -q -p xtask -- probe src/android.rs \
  /tmp/android-lane/arm.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"' \
  --cfg 'feature="android_api_lt_21"'
```

→ **exit 0, 0 errors**, on the slice's **first** probe: the two new `FIND_METHOD` macros, the one
`GetStaticMethodID` call of C 2043-2046 and the `GetFieldID` of C 2079-2082 all read correctly.

**⑥ The checklist reading.**  `extract-abi.py --check`: **117 `missing function`** rows (down from
120 - the three tables retire), the same **2 `missing file-scope object`** rows, **3 `invented
exported symbol`** rows (Owed 9's class), **0 `does not mention`**, and the line-count row at
**68.9%** (C 7,594 against the port's 2,364).

### Deviations this slice adds

- **C 2024-2048 is the one class table without a `FIND_METHOD` macro**: it fetches a *static*
  method (`GetStaticMethodID`, C 2043-2046) in one statement pair, so the port writes it directly
  rather than through the macro.
- **C 2050-2084 mixes both devices**: one macro call (`destroy_handle`) and then a direct
  `GetFieldID` for the `handle` field (C 2079-2082), which the port keeps in C's order.
- **The `org/femacs/` rename** covers this slice's two `org/gnu/emacs/` class names
  (`EmacsCursor` at C 2001, `EmacsHandleObject` at C 2057); C 2031's `android/view/KeyCharacterMap`
  is an Android platform class and keeps its name.

## 2026-10-12: slice 5 (C 2194-2253) - the shutdown and low-memory entries

`NATIVE_NAME (emacsAbort)`, `NATIVE_NAME (quit)`, `android_shut_down_emacs`,
`NATIVE_NAME (shutDownEmacs)`, `android_on_low_memory` and `NATIVE_NAME (onLowMemory)` - and the
record of the one row this slice **steps over**.

### Evidence (this slice's readings)

**⑦ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors**
after **two** probe runs: the first read `__func__`'s two uses (the unit's own device is a local
`let __func__: *const c_char = c"name".as_ptr ();`, C 614/629's shape) and `Fclear_image_cache`'s
two arguments (`Qt`/`Qnil` are *functions* in this tree, so C's bare `Qt` is `Qt ()`).

**⑧ The checklist reading.**  `extract-abi.py --check`: **115 `missing function`** rows (down from
117 - the two plain `static`s retire), the same **2 `missing file-scope object`** rows, **7
`invented exported symbol`** rows (Owed 9's class - the four `NATIVE_NAME` entries of this slice add
four, which is the **androidterm slice-15 finding** applying here too: a `NATIVE_NAME`-defined
function never enters the checker's function inventory, so those four had no `missing function` row
to retire), **0 `does not mention`**, and the line-count row at **67.6%** (C 7,594 against the
port's 2,464).

### Deviations this slice adds

- **`__func__`** (C 2207/2222) is a local `let __func__: *const c_char = c"…".as_ptr ();` in each
  function that logs, C's own `__func__` spelling kept (the unit's C 614/629 device).
- **C 2232/2252's `android_run_in_emacs_thread (worker, NULL)`** is
  `Some (worker)` + `std::ptr::null_mut ()`, and the two workers are `unsafe extern "C" fn`
  because their address is taken.
- **C 2243's `Fclear_image_cache (Qt, Qnil)`** is `Fclear_image_cache (Qt (), Qnil ())`: this tree
  spells the two built-in symbols as functions (the `globals.rs` `Qnil`/`Qt` pair).
- **`ANDROID_LOG_VERBOSE`** (C 2207) is added to the unit's measured NDK enum with the derivation
  recorded (VERBOSE is two steps below INFO, whose value the unit already measured).

## 2026-10-12: slice 6 (C 2255-2356) - the five event posters

`NATIVE_NAME (sendConfigureNotify)`, `sendKeyPress`, `sendKeyRelease`, `sendFocusIn` and
`sendFocusOut`.

### Evidence (this slice's readings)

**⑨ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors**, on
the slice's **first** probe: the three union arms (`xconfigure`, `xkey`, `xfocus`), the seven field
types C's `jlong`/`jint` arguments narrow into, and the `event_serial` arithmetic all read
correctly.

**⑩ The checklist reading.**  `extract-abi.py --check`: the `missing function` count is
**unchanged at 115** - all five definitions are `NATIVE_NAME` entries, and (the androidterm
slice-15 finding, recorded here in Owed 9's note) a `NATIVE_NAME`-defined function never enters the
checker's inventory, so it has no row to retire; the five new **`invented exported symbol`** rows
bring that count to **12**, the two `missing file-scope object` rows stand, there is **0
`does not mention`**, and the line-count row reads **65.9%** (C 7,594 against the port's 2,586).

### Deviations this slice adds

- **The five events are `std::mem::zeroed ()` then written field by field** (PORTING.md 1.5, C's
  `union android_event event;`), with the union member's Rust spelling `xconfigure.r#type` for C's
  `.type`.
- **C 2266-2272/2289-2295/…/2350-2352's `++event_serial`** is the two-statement form (the `c_uint`
  write, then the read), and the `as c_ulong`/`as android_window`/`as android_time`/`as c_uint`
  conversions are C's own promotions into the record's member types.
- **C 2355's `return ++event_serial;`** is the same two statements with the return last, so the
  returned value is the incremented serial (unlike the four others, which return the value they
  stored).

## 2026-10-12: slice 7 (C 2358-2548) - the nine event posters

`NATIVE_NAME (sendWindowAction)`, `sendEnterNotify`, `sendLeaveNotify`, `sendMotionNotify`,
`sendButtonPress`, `sendButtonRelease`, `sendTouchDown`, `sendTouchUp` and `sendTouchMove`.

### Evidence (this slice's readings)

**⑪ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors**,
on the slice's **second** probe: the first read eight `E0308` rows at the four `time` members (see
the deviation below), and with that one spelling corrected the five union members (`xaction`,
`xcrossing`, `xmotion`, `xbutton`, `touch`), the `jlong`/`jint`-into-member conversions and the
`event_serial` arithmetic all read correctly - 11 warnings, the same FFI-safety and
`runtime read/write symbol` set the lane's baseline carries.

**⑫ The checklist reading.**  `extract-abi.py --check`: the `missing function` count is
**unchanged at 115** - all nine definitions are `NATIVE_NAME` entries, and (the androidterm
slice-15 finding, recorded here in Owed 9's note) a `NATIVE_NAME`-defined function never enters the
checker's inventory, so it has no row to retire; the nine new **`invented exported symbol`** rows
bring that count to **21**, the two `missing file-scope object` rows stand, there is **0
`does not mention`**, and the line-count row reads **63.1%** (C 7,594 against the port's 2,805).

### Deviations this slice adds

- **The nine events are `std::mem::zeroed ()` then written field by field** (PORTING.md 1.5, C's
  `union android_event event;`), with the union member's Rust spelling `xaction.r#type` for C's
  `.type`, exactly as slices 5-6 spell theirs.
- **C's four `unsigned long` members take `time as c_ulong`, not `time as android_time`.**
  `androidgui.h` 44 typedefs `android_time` as `unsigned int`, and the crossing/motion/button/touch
  records are the `unsigned long` ones (C 378/387/396/432/456; C 315/344/354 are the three
  `android_time` members slices 5-6 write), so the faithful conversion is the `c_ulong` one - the
  mirror already declares those four members `c_ulong` (`femacs/src/include/androidgui.rs`
  444/456/468/511).  The first probe's eight `E0308` rows were that spelling, not the mirror.
- **C's `++event_serial`** is the two-statement form (the `c_uint` write, then the read), and the
  `as c_ulong`/`as android_window`/`as c_uint` conversions are C's own promotions into the record's
  member types.  All nine return the value they stored (none has C 2355's `return ++event_serial;`
  shape, which is the previous slice's).

## 2026-10-12: slice 8 (C 2550-2664) - the six remaining simple posters

`NATIVE_NAME (sendWheel)`, `sendIconified`, `sendDeiconified`, `sendContextMenu`, `sendExpose` and
`sendDndDrag`.

### Evidence (this slice's readings)

**⑬ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors, 11
warnings**, on the slice's **first** probe: the six union members (`wheel`, `iconified` - written by
two of the six - `menu`, `xexpose`, `dnd`), the `jfloat`-into-`double` pair of C 2567-2568 and the
`null_mut ()`/`0` pair of C 2659-2660 all read correctly.

**⑭ The checklist reading.**  `extract-abi.py --check`: the `missing function` count is
**unchanged at 115** (the same Owed 9 reason ⑫ records - all six are `NATIVE_NAME` definitions);
the six new **`invented exported symbol`** rows bring that count to **27**, the two `missing
file-scope object` rows stand, there is **0 `does not mention`**, and the line-count row reads
**61.3%** (C 7,594 against the port's 2,937).

### Deviations this slice adds

- **C 2567-2568's `float`-into-`double` pair** is the one conversion this slice adds: the JNI
  parameters are `jfloat` and `android_wheel_event`'s members are `double` (`androidgui.h`'s wheel
  record, mirror `femacs/src/include/androidgui.rs` 545-546), so the port writes
  `x_delta as f64`/`y_delta as f64` - C's own default argument promotion.
- **C 2659-2660's `NULL`/`0` pair** is `std::ptr::null_mut ()` and `0` in the `dnd` member (the
  `*mut c_ushort`/`size_t` pair), the spelling the unit's own C 145-158 declarations already use.
- **`sendIconified`/`sendDeiconified` write the same `iconified` union member** with the two
  different types (C 2582/2598), exactly as C does - the member is one record, the two entries
  differ only in the type constant.

## 2026-10-12: slice 9 (C 2666-2748) - the two JNI string entries

`NATIVE_NAME (sendDndUri)` and `NATIVE_NAME (sendDndText)`.

### Evidence (this slice's readings)

**⑮ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors, 11
warnings**, on the slice's **first** probe: the three `JNIEnv` function-table reads through the
mirror's `Option<unsafe extern "C" fn …>` members (`include/jni.rs` 302-304), the
`jsize`-into-`size_t` pair of the two `malloc` calls, the `*mut c_ushort` local and the two
`return 0;` arms all read correctly.

**⑯ The checklist reading.**  `extract-abi.py --check`: the `missing function` count is
**unchanged at 115** (the same Owed 9 reason ⑫/⑭ record), the two new **`invented exported
symbol`** rows bring that count to **29**, the two `missing file-scope object` rows stand, there is
**0 `does not mention`**, and the line-count row reads **60.1%** (C 7,594 against the port's
3,031).

### Deviations this slice adds

- **C's `(*env)->Method (env, …)` is the port's `((*(*env)).Method.unwrap ()) (env, …)`** - the
  `JNIEnv` is the NDK mirror's `*const JNINativeInterface` and the table's entries are `Option`
  fields (`include/jni.rs` 302-304), the shape `femacs/src/androidterm.rs` 6228/6234/6251 already
  carries for the same three members.  The `*mut JNIEnv` parameter C declares is the mirror's own
  pointer, so no cast is needed at the call.
- **C 2685/2727's `malloc (length * sizeof *buffer)`** is
  `malloc (length as usize * std::mem::size_of::<c_ushort> ()) as *mut c_ushort`: the `jsize`
  operand takes C's own conversion to `size_t`, and the `*mut c_void` result C relies on implicitly
  is spelled out (the androidterm.rs 6229 device).
- **C's two inner comments are carried verbatim** with their C line markers: `/* Out of memory.  */`
  (C 2687/2729) and the two-line JVM note (C 2694-2695/2736-2737), the androidvfs.rs 7138 shape.
- **C 2698/2740's `memcpy`** writes both conversions out (`as *mut c_void` / `as *const c_void`)
  and C's `length * sizeof *buffer` becomes the same `usize` product as the `malloc` above.
- **The two functions are textually identical apart from the type constants**
  (`ANDROID_DND_URI_EVENT`/`ANDROID_DND_TEXT_EVENT`), and the port keeps them as two full
  definitions rather than a shared helper - C has two definitions, so the port does.

## 2026-10-12: slice 10 (C 2750-2888) - the notification, configuration and query entries

`NATIVE_NAME (sendNotificationDeleted)`, `sendNotificationAction`, `sendConfigurationChanged`,
`shouldForwardMultimediaButtons`, `getQuitKeycode` and `shouldForwardCtrlSpace`.

### Evidence (this slice's readings)

**⑰ The arm flip of this slice.**  The same three-`--cfg` command ③ uses → **exit 0, 0 errors, 11
warnings**, on the slice's **second** probe.  The first read one `E0428` (`strdup` defined twice):
this slice's own declaration was a duplicate - the unit *already* declares `strdup` in its later
declaration block (the "`string.in.rs` 194-199 keeps only the dead `_strdup` redirect" note), so the
extra block was removed and nothing else changed.  With that, the `GetStringUTFChars`/
`ReleaseStringUTFChars` pair, the `const void *characters` local that C's two `GetString*` calls
share, the configuration `match` and the three `jboolean`/`jint` returns all read correctly.

**⑱ The checklist reading.**  `extract-abi.py --check`: the `missing function` count is
**unchanged at 115** (all six are `NATIVE_NAME` definitions - the Owed 9 class ⑫/⑭/⑯ record), the
six new **`invented exported symbol`** rows bring that count to **35**, the two `missing
file-scope object` rows stand, there is **0 `does not mention`** (C 2844's `switch` loses the four
tokens the macro-body-notes rows of `android_write_event`/`android_detect_focus_change` record for
plain functions, but this entry is a `NATIVE_NAME` definition and therefore not in the extractor's
`ci.functions` - the same blindness that keeps its row count unchanged), and the line-count row
reads **58.0%** (C 7,594 against the port's 3,193).

### Deviations this slice adds

- **C 2844's `switch (detail)` is the port's `match`**, with C's two labels and its `default` kept
  in C's order and an explanatory comment above it (the `android_detect_focus_change` shape,
  `femacs/src/androidterm.rs` 782-790); the four tokens `switch`/`case`/`default`/`break` have no
  Rust counterpart, and C 2845/2850/2854/2858's braces and jumps have no line of their own.
- **C 2788's `const void *characters`** is the one local C reuses for both `GetStringUTFChars`
  (`*const c_char`) and `GetStringChars` (`*const jchar`), so the port writes the two conversions
  out at the call sites (`as *const c_void`, then `as *const c_char`/`as *const jchar` where C's
  implicit `void *` conversion did the work) instead of splitting the local.
- **C 2808's `malloc` has no null check** and the port adds none - C 2809's `GetStringChars` is the
  first test C makes, and the `xfree` arm at C 2811-2817 is the only cleanup path.
- **C 2757/2788's `characters` and C 2789's `length`** keep C's declaration order inside the
  function (locals first, then the event writes), and `getQuitKeycode` is the slice's one entry
  without `JNI_STACK_ALIGNMENT_PROLOGUE` because C has none there.
- **`return !x;` on a `bool`** is `(! x) as jboolean` (the `androidvfs.rs` 4170 shape), and C 2879's
  `(jint) android_quit_keycode` is `android_quit_keycode as jint` (`EMACS_INT` -> `jint`).
