# emacs/src/msdos.c (4,350 lines) — slices 1-3: C 1-3468

> **One of P-86's class: the frozen configuration compiles nothing here.**  The whole body sits
> inside C 34's `#ifdef MSDOS`; `MSDOS` is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` 2849, and the MS-DOS (djgpp) build is the
> `msdos/` subdirectory's own: `emacs/msdos/sed1v2.inp` 136 rewrites the generated
> `src/Makefile.in` 307's empty `MSDOS_OBJ` into `= dosfns.o msdos.o w16select.o`, whose macro is
> `MSDOS` (`configure.ac` 6953's `AH_TEMPLATE`).  `docs/baseline/frozen-build-sources.txt` lists
> no `msdos.o` for this target, so `tools/extract-abi.py`'s checklist for the file is empty (C
> 1-4350 reads `## Subrs (DEFUN) — 0`, `## Functions defined — 0`, `## Macros defined — 0`,
> `## Types and their fields — 0`, `## File-scope objects — 0`, and `--check` therefore passes
> vacuously — P-86 ②/⑩).  What follows is P-86 ②'s substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `femacs/src/msdos.rs`
> (**4562 lines**, 173,728 bytes, md5 `d0cddfc0e226e2ebd807788e2cd585d9`), carrying **C 1-3468**
> with **both arms of every `#if`** (slice 1: C 1-1295; slice 2: C 1296-2422; slice 3: C 2423-3468).
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the
> frozen configuration does not compile the file ⇒ the port is the empty unit plus the note that
> names the gate" (the P-08 reading).  P-86 ① replaces that with *transcribe the text, both arms,
> no stub*; what the earlier note measured (not in the frozen source list, empty checklist, a
> 336-byte object) is unchanged and still true — it is the *conclusion* that is withdrawn, exactly
> as `docs/ports/w32proc.md`, `docs/ports/w32.md` and `docs/ports/androidvfs.md` record for their
> own files.  The reference is **not valid UTF-8** (it is `coding: cp850`); it is read with
> `errors="replace"` and the eight non-ASCII lines are all at C 1972-2011 — slice 2's keyboard
> maps, which the port carries as `\xNN` escapes so that its own text stays ASCII (Deviations 14).
>
> **These are P-86 ④'s first two slices: C 1-1295 and C 1296-2422.**  Slice 1 runs from the
> file's first line through `IT_update_end`'s closing brace (C 1288-1295): the notes and the
> include list, the `SYS_ENVIRON` pair, the allocator triple, the whole mouse-control half
> (`event_timestamp`, `mouse_on`/`mouse_off`, `mouse_setup_buttons`, the `msdos-set-mouse-buttons`
> subr, `mouse_get_xy`, `mouse_moveto`, `mouse_pressed`/`mouse_released`/`mouse_button_depressed`,
> `mouse_get_pos`, `mouse_check_moved`, `mouse_clear_clicks`, `mouse_init`) and the first half of
> the screen half (both `#ifndef HAVE_X_WINDOWS` blocks' opening items,
> `dosv_refresh_virtual_screen`, `dos_direct_output`, `bright_bg`, `maybe_enable_blinking`,
> `vga_installed`, `dos_set_window_size`, `mouse_off_maybe`, `msdos_set_cursor_shape`,
> `IT_set_cursor_type`, `IT_ring_bell`, `IT_set_face`, `IT_write_glyphs`, `mouse_preempted`,
> `popup_activated`, `IT_write_glyphs_with_face`, `tty_draw_row_with_mouse_face`,
> `IT_clear_end_of_line`, `IT_clear_screen`, `IT_clear_to_end`, `IT_cursor_to`,
> `IT_display_cursor`, `IT_cmgoto`, `IT_update_begin`, `IT_update_end`).
>
> **Slice 2 is C 1296-2422.**  It opens with C 1296's blank line and runs through the closing brace
> of the `recent-doskeys` subr (C 2403-2422): the rest of the second `#ifndef HAVE_X_WINDOWS`
> block (`IT_frame_up_to_date`, `IT_copy_glyphs`, `IT_insert_glyphs`, `IT_delete_glyphs`,
> `IT_set_terminal_modes`, `IT_reset_terminal_modes`, the `msdos-remember-default-colors` subr and
> `IT_set_frame_parameters`), then the file-gated `internal_terminal_init`,
> `initialize_msdos_display` and `dos_get_saved_screen`, then the whole keyboard-control half: the
> ten scan-code class bits, `extended_kbd`, `struct kbd_translate`, `struct dos_keyboard_map`, the
> five keyboard maps (`us`, `fr`, `it`, `dk`, `jp`), the two translate tables,
> `keyboard_layout_list`, `keyboard`/`keyboard_map_all`/`international_keyboard`,
> `dos_set_keyboard`, `keypad_translate_map`, `grey_key_translate_map`, the 167-entry
> `ibmpc_translate_map`, the nine BIOS modifier bits, `dos_get_modifiers`, `NUM_RECENT_DOSKEYS`,
> the three `recent_doskeys` objects and the `recent-doskeys` subr.
>
> **Slice 3 is C 2423-3468.**  It opens with C 2423's blank line and C 2424's banner and runs
> through the closing brace of `rootrelativepath` (C 3459-3468): `dos_rawgetc` (C 2425-2759, the
> BIOS key reader with its two `#ifndef HAVE_X_WINDOWS` arms and the mouse-button half),
> `prev_get_char` with `dos_keysns`/`dos_keyread` (C 2761-2785), the whole MS-DOS X menu
> simulation inside C 2787's `#ifndef HAVE_X_WINDOWS` block (`menu_help_message` and its three
> neighbours, `IT_menu_create`, `IT_menu_make_room`, `IT_menu_search_pane`, `IT_menu_calc_size`,
> the file-local `BUILD_CHAR_GLYPH` macro, `IT_menu_display`, `IT_menu_state`, the six XMenu
> façade functions `XMenuCreate`/`XMenuAddPane`/`XMenuAddSelection`/`XMenuLocate`/
> `XMenuActivate`/`XMenuDestroy`, C 3303's `#endif`) and the DOS/UNIX conversion layer
> (`dostounix_filename`, `unixtodos_filename`, `getdefdir`, `emacs_root_dir`, `crlf_to_lf`, the
> two `msdos-*file-name*` subrs, `msdos_downcase_filename`, `emacsroot` and `rootrelativepath`).
> **The next live definition is C 3475-3654's `init_environment`** (C 3470-3473 is its banner),
> and the remainder (C 3469-4350) is the table's last row: `init_environment`, the DOS/V screen
> buffer, the subprocess layer and `syms_of_msdos`, none of it ported here.
>
> **The file gate is `feature = "msdos"`** (C 34's `MSDOS`), on every item *and on the include
> graph* (P-86 ⑧: an ungated `include!` breaks the frozen build at once, which is why the reading
> in Evidence ① is an empty object).
>
> **The DJGPP runtime surface is declared in this unit (P-88 ①), and this batch is `text-only`
> (P-86 ③⑵).**  `<dos.h>` (C 45), `<dpmi.h>` (C 57), `<sys/farptr.h>` (C 58),
> `<go32.h>` (C 86), `<pc.h>` (C 87), `<conio.h>` (C 60), `<dir.h>` (C 51), `<io.h>` (C 56),
> `<libc/dosio.h>` (C 59) and `<crt0.h>` (C 117) have no file in the reference tree and **no tree
> on this machine**: there is **no DJGPP toolchain** (`which -a i586-pc-msdosdjgpp-gcc
> i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty; nothing named `djgpp`/`go32` under `/opt/homebrew`,
> `/usr/local` or `/Library`), so there is **no arm flip on a DJGPP target** and **no measurement
> of any record, constant or prototype from those headers** — every size, offset and value of
> theirs is **UNVERIFIED** and owed (see `## Owed`).  The C 59 `<libc/dosio.h>` comment and the
> reference's own provenance comments at C 49-60 are what name those headers; `<go32.h>` and
> `<pc.h>` carry no such comment, so even *which* of the two supplies `ScreenPrimary`/`_go32_info_block`
> is unmeasured and said so in the unit.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`#ifdef MSDOS`, C 34) | `feature = "msdos"` (on every item *and* on C 35-107's include graph) | the MS-DOS (djgpp) build |
| C 62 `#if (__DJGPP__ + (__DJGPP_MINOR__ > 3)) >= 3` (`SYS_ENVIRON`) | true arm `not (feature = "djgpp_2_3")`, else arm `feature = "djgpp_2_3"` (registered, Cargo.toml 62) | djgpp 2.4+ / djgpp 2.3 and older |
| C 95 `#ifndef _USE_LFN` | `not (feature = "_use_lfn")`; the `#else` side is `<libc/dosio.h>`'s own definition, carried by that header | `<libc/dosio.h>` does not define `_USE_LFN` |
| C 99 `#ifndef _dos_ds` | `not (feature = "_dos_ds")`; the other side is `<go32.h>`'s definition | `<go32.h>` does not define `_dos_ds` |
| C 109 `#ifndef SYSTEM_MALLOC` | `feature = "no_system_malloc"` (registered, Cargo.toml 438 — C's `!SYSTEM_MALLOC`); no `#else` in C | the MS-DOS build's own allocator |
| C 111 `#ifdef GNU_MALLOC` | `feature = "gnu_malloc"` (**not registered** — Owed 1); no `#else` in C | `GNU_MALLOC` |
| C 119 `#ifdef REL_ALLOC` / C 121 `#else` | `feature = "rel_alloc"` (registered, Cargo.toml 490) / `not (feature = "rel_alloc")` | `REL_ALLOC` on/off |
| C 402 `#ifndef HAVE_X_WINDOWS` … C 478 `#endif` | `all (feature = "msdos", not (feature = "have_x_windows"))` | no window system (the MS-DOS build's case) |
| C 480 `#ifndef HAVE_X_WINDOWS` … C 1771 `#endif` | the same pair, on every item of C 482-1771 (slices 1 and 2) | the same |
| C 1171 `#if 0` … C 1211 `#endif` (inside `IT_cmgoto`) | dead in *every* configuration; C's text is kept as a comment (the `src/cm.rs` 86 / `src/character.rs` 118 `#if 0` shape) | never |
| C 1784 `#ifdef HAVE_X_WINDOWS` … C 1787 `#endif` (inside `internal_terminal_init`) | `all (feature = "msdos", feature = "have_x_windows")`; C has no `#else` arm, so the other side is empty text | a window-system build |
| C 1796 `#ifndef HAVE_X_WINDOWS` … C 1870 `#endif` (the same function's body) | `all (feature = "msdos", not (feature = "have_x_windows"))`; no `#else` in C | the MS-DOS build |
| C 1907 `#ifndef HAVE_X_WINDOWS` / C 1912 `#else` / C 1914 `#endif` (inside `dos_get_saved_screen`) | **both arms carried**: `not (feature = "have_x_windows")` and `feature = "have_x_windows"` | either build |
| C 2433 `#ifndef HAVE_X_WINDOWS` … C 2436 `#endif` (inside `dos_rawgetc`, `IT_cmgoto`) | `all (feature = "msdos", not (feature = "have_x_windows"))`; C has no `#else` arm, so the other side is empty text | the MS-DOS build |
| C 2465 `#ifndef HAVE_X_WINDOWS` … C 2473 `#endif` (the same function, `Vdos_display_scancodes`) | the same pair | the same |
| C 2787 `#ifndef HAVE_X_WINDOWS` … C 3303 `#endif` (the menu simulation and the six XMenu façade functions) | the same pair, on every item of C 2789-3302 | the same |

**Sixteen of the file's 27 conditions are inside the ported span** — C 34, C 62, C 95, C 99,
C 109, C 111, C 119, C 402, C 480 and C 1171 (slice 1's ten), slice 2's C 1784, C 1796 and C 1907,
and slice 3's C 2433, C 2465 and C 2787; the remaining eleven are past C 3468 and each is named in
Owed 9.  C 1171's `#if 0` is the one with no arm at all (see its row).

`have_x_windows` (Cargo.toml 182), `rel_alloc` (490), `msdos` (223) and `no_system_malloc` (438)
are registered; `gnu_malloc`, `_use_lfn` and `_dos_ds` are not (Owed 1).  The arm flip also needs
**`dispextern_face_records`** (Cargo.toml 61): the mirror's `struct face`, `Mouse_HLInfo`'s
bit accessors and `FACE_FROM_ID_OR_NULL` live behind it (`include/dispextern.rs` 2135,
`include/frame.rs` 1485), which is a *port-side* ruling rather than a C condition — the same
shape as `docs/ports/w32proc.md`'s "the file gate is `windowsnt` **and** the same build defines
`HAVE_NTGUI`" (Owed 4/5).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-33 | the file's own notes (C 20-30), C 32's `#include <config.h>` | ungated (the port carries the notes above `include/config.rs`) |
| 34-61 | `#ifdef MSDOS` and the include list (C 35-60) | file gate; C 45/51/56/57/58/59/60 are the DJGPP stand-in section |
| 62-67 | the `SYS_ENVIRON` pair (C 62-66) | file gate; arms `not djgpp_2_3` / `djgpp_2_3` |
| 68-108 | the Emacs include list (C 68-107) | file gate |
| 109-129 | the allocator triple (C 109-126) and `event_timestamp`'s banner | file gate; `no_system_malloc` / `gnu_malloc` / `rel_alloc` + its `#else` |
| 130-144 | `event_timestamp` (C 130-142) | file gate |
| 145-164 | the mouse-control banner, `NUM_MOUSE_BUTTONS` (C 153), the mouse globals (C 155-162) | file gate |
| 165-181 | `mouse_on` (C 165-179) | file gate |
| 182-198 | `mouse_off` (C 182-196) | file gate |
| 199-215 | `mouse_setup_buttons` (C 199-214) | file gate |
| 216-236 | the `msdos-set-mouse-buttons` subr (C 216-234) | file gate |
| 237-248 | `mouse_get_xy` (C 237-246) | file gate |
| 249-265 | `mouse_moveto` (C 249-263) | file gate |
| 266-284 | `mouse_pressed` (C 266-282) | file gate |
| 285-299 | `mouse_released` (C 285-297) | file gate |
| 300-317 | `mouse_button_depressed` (C 300-315) | file gate |
| 318-337 | `mouse_get_pos` (C 318-335) | file gate |
| 338-350 | `mouse_check_moved` (C 338-346) | file gate |
| 351-364 | `mouse_clear_clicks` (C 351-362) | file gate |
| 365-395 | `mouse_init` (C 365-394) | file gate |
| 396-401 | the screen-control banner and `internal_terminal` (C 400) | file gate |
| 402-449 | the first `#ifndef HAVE_X_WINDOWS` (C 402), `ScreenAttrib` (C 403), the screen globals, `dosv_refresh_virtual_screen`'s banner | `not have_x_windows` |
| 450-464 | `dosv_refresh_virtual_screen` (C 450-462) | `not have_x_windows` |
| 465-481 | `dos_direct_output` (C 465-477) and C 478's `#endif` | `not have_x_windows` |
| 482-485 | the second `#ifndef HAVE_X_WINDOWS` (C 480) and `blink_bit` (C 482) | `not have_x_windows` |
| 486-502 | `bright_bg` (C 486-498) | `not have_x_windows` |
| 503-516 | `maybe_enable_blinking` (C 503-513) | `not have_x_windows` |
| 517-531 | `vga_installed` (C 517-526) and `dos_set_window_size`'s banner | `not have_x_windows` |
| 532-637 | `dos_set_window_size` (C 532-632) and the `mouse_off_maybe` comment (C 634-636) | `not have_x_windows` |
| 638-659 | `mouse_off_maybe` (C 638-650) and the three cursor constants (C 652-654) | `not have_x_windows` |
| 660-740 | `msdos_set_cursor_shape` (C 660-738) | `not have_x_windows` |
| 741-782 | `IT_set_cursor_type` (C 741-780) | `not have_x_windows` |
| 783-808 | `IT_ring_bell` (C 783-797) and `it_face_override` (C 801) | `not have_x_windows` |
| 809-877 | `IT_set_face` (C 809-873) and the `MAX_SCREEN_BUF` banner | `not have_x_windows` |
| 878-883 | `MAX_SCREEN_BUF` (C 878) and `encode_terminal_code`'s declaration (C 880-881) | `not have_x_windows` |
| 884-969 | `IT_write_glyphs` (C 884-964) and the Mouse-Highlight banner (C 966-968) | `not have_x_windows` |
| 970-981 | `mouse_preempted` (C 970) and `popup_activated` (C 973-976) | `not have_x_windows` |
| 982-993 | `IT_write_glyphs_with_face` (C 982-988) | `not have_x_windows` |
| 994-1035 | `tty_draw_row_with_mouse_face` (C 994-1033) | `not have_x_windows` |
| 1036-1067 | `IT_clear_end_of_line` (C 1036-1065) | `not have_x_windows` |
| 1068-1091 | `IT_clear_screen` (C 1068-1089) | `not have_x_windows` |
| 1092-1106 | `IT_clear_to_end` (C 1092-1104) | `not have_x_windows` |
| 1107-1119 | `IT_cursor_to` (C 1107-1115) and `cursor_cleared` (C 1117) | `not have_x_windows` |
| 1120-1161 | `IT_display_cursor` (C 1120-1140) and the `IT_cmgoto` banner (C 1142-1160) | `not have_x_windows` |
| 1162-1230 | `IT_cmgoto` (C 1162-1228; C 1171-1211 is the `#if 0`) | `not have_x_windows`; the `#if 0` is dead in every configuration |
| 1231-1287 | `IT_update_begin` (C 1231-1285) | `not have_x_windows` |
| 1288-1295 | `IT_update_end` (C 1288-1295) | `not have_x_windows` |
| 1296-1333 | C 1296's blank line and `IT_frame_up_to_date` (C 1297-1333) | `not have_x_windows` |
| 1334-1382 | `IT_copy_glyphs` (C 1339-1368) and `IT_insert_glyphs` (C 1371-1382) | `not have_x_windows` |
| 1383-1388 | `IT_delete_glyphs` (C 1384-1388) | `not have_x_windows` |
| 1389-1460 | the `xfaces.c` note (C 1390-1393) and `IT_set_terminal_modes` (C 1395-1460) | `not have_x_windows` |
| 1461-1540 | `IT_reset_terminal_modes` (C 1462-1540) | `not have_x_windows` |
| 1541-1562 | the `msdos-remember-default-colors` subr (C 1542-1562) | `not have_x_windows` |
| 1563-1767 | `IT_set_frame_parameters` (C 1564-1767) | `not have_x_windows` |
| 1768-1771 | C 1769's `extern void init_frame_faces (struct frame *);` (dropped, PORTING.md 0.1-12) and C 1771's `#endif` (the C 480 block ends) | `not have_x_windows` |
| 1772-1871 | `internal_terminal_init` (C 1774-1871) | file gate; arms C 1784 `#ifdef HAVE_X_WINDOWS` and C 1796 `#ifndef HAVE_X_WINDOWS` |
| 1872-1902 | `initialize_msdos_display` (C 1873-1902) | file gate |
| 1903-1915 | `dos_get_saved_screen` (C 1904-1915) | file gate; arms C 1907 `#ifndef` / C 1912 `#else` |
| 1916-1958 | the keyboard banner (C 1917-1927), the ten scan-code `#define`s (C 1929-1940), `extended_kbd` (C 1942), `struct kbd_translate` (C 1944-1948), `struct dos_keyboard_map` (C 1950-1956) | file gate |
| 1959-2045 | the five keyboard maps and their translate tables (C 1959-2032) and `keyboard_layout_list` (C 2034-2045) | file gate |
| 2046-2080 | `keyboard`/`keyboard_map_all`/`international_keyboard` (C 2047-2049) and `dos_set_keyboard` (C 2051-2080) | file gate |
| 2081-2112 | `keypad_translate_map` (C 2082-2100) and `grey_key_translate_map` (C 2102-2112) | file gate |
| 2113-2305 | `ibmpc_translate_map` (C 2114-2305; 167 entries) | file gate |
| 2306-2316 | C 2306's form feed and the nine BIOS modifier bit `#define`s (C 2308-2316) | file gate |
| 2317-2396 | `dos_get_modifiers` (C 2318-2396) | file gate |
| 2397-2422 | `NUM_RECENT_DOSKEYS` and the three `recent_doskeys` objects (C 2398-2401) and the `recent-doskeys` subr (C 2403-2422) | file gate |
| 2423-2432 | C 2423's blank line, C 2424's banner and `dos_rawgetc`'s declarations (C 2425-2431) | file gate |
| 2433-2437 | `dos_rawgetc`'s first `#ifndef HAVE_X_WINDOWS` (C 2433) and the BIOS poll loop's opening (C 2438-2442) | file gate; arm C 2433 |
| 2438-2463 | the key read, the `recent_doskeys` ring (C 2453-2461) and `dos_get_modifiers`'s call (C 2463) | file gate |
| 2464-2520 | `dos_rawgetc`'s second `#ifndef HAVE_X_WINDOWS` (C 2465-2467) and the private/general translate-table lookup (C 2475-2519) | file gate; arm C 2465 |
| 2521-2542 | the `c == 0` modifier block (C 2521-2541) | file gate |
| 2543-2635 | the fall-through `switch (code & 0xf000)` (C 2543-2635) | file gate |
| 2636-2656 | the `code == 0` skip, the mouse-highlight hide and the `kbd_buffer_store_event` (C 2637-2655) | file gate |
| 2657-2759 | `dos_rawgetc`'s mouse half (C 2658-2756) and its `return -1` (C 2758) | file gate |
| 2760-2786 | `prev_get_char` (C 2761), `dos_keysns` (C 2764-2771) and `dos_keyread` (C 2774-2785) | file gate |
| 2787-2801 | C 2787's `#ifndef HAVE_X_WINDOWS`, the block's banner (C 2789-2795) and the four menu-help objects (C 2798-2801) | `not have_x_windows` |
| 2802-2876 | `IT_menu_create` (C 2803-2811), `IT_menu_make_room` (C 2816-2835), `IT_menu_search_pane` (C 2839-2854) and `IT_menu_calc_size` (C 2858-2876) | `not have_x_windows` |
| 2877-2962 | the file-local `BUILD_CHAR_GLYPH` (C 2880-2887) and `IT_menu_display` (C 2889-2962) | `not have_x_windows` |
| 2963-3047 | the six-function XMenu façade's first four (`XMenuCreate` C 2968-2972, `XMenuAddPane` C 2978-3004, `XMenuAddSelection` C 3008-3035, `XMenuLocate` C 3039-3047) | `not have_x_windows` |
| 3048-3055 | `struct IT_menu_state` (C 3049-3055) | `not have_x_windows` |
| 3056-3282 | `XMenuActivate` (C 3060-3282) | `not have_x_windows` |
| 3283-3303 | `XMenuDestroy` (C 3286-3302) and C 3303's `#endif` (the block ends) | `not have_x_windows` |
| 3304-3310 | the DOS/UNIX banner (C 3305) and C 3307's `msdos_downcase_filename` prototype (dropped, PORTING.md 0.1-12) | file gate |
| 3311-3341 | `dostounix_filename` (C 3311-3322) and `unixtodos_filename` (C 3326-3341) | file gate |
| 3342-3370 | `getdefdir` (C 3345-3370) | file gate |
| 3371-3380 | `emacs_root_dir` (C 3372-3380) | file gate |
| 3381-3404 | `crlf_to_lf` (C 3384-3404) | file gate |
| 3405-3412 | the `msdos-long-file-names` subr (C 3406-3412) | file gate |
| 3413-3435 | `msdos_downcase_filename` (C 3416-3435) | file gate |
| 3436-3453 | the `msdos-downcase-filename` subr (C 3437-3453) | file gate |
| 3454-3468 | `emacsroot` (C 3457) and `rootrelativepath` (C 3459-3468) | file gate |
| 3469-4350 | the unported remainder (C 3470-4350; `init_environment` onward) | not ported this batch |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos3/cargo cargo run -q -p xtask -- build-tu src/msdos.c`
→ `built src/msdos.rs -> ../build/femacs/msdos.o`, **exit 0**, and `build/femacs/msdos.o` is
**336 bytes** (sha256 `d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed` — the same
reading all three slices took, which is what P-86 ⑧ predicts).  Every item of C 1-3468 sits behind
a `msdos` gate, so under the frozen feature set rustc compiles only `include/config.rs` and the
`#![allow]` attribute; an `include!` outside the gate would break this compile at once.  (The
first run of this batch read `not a valid postfix operator` at the port's line 3306 — a C `++`
that leaked through the transcription; it was fixed before this reading.)

**② The `msdos` arm flip, as a per-file error census (P-86 ③⑴/⑦).**  There is no DJGPP target to
compile for, so this is rustc on *this* machine with the derived feature set:
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos3/cargo cargo run -q -p xtask -- probe src/msdos.rs
/tmp/msdos_arm12.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"' --cfg
'feature="dispextern_face_records"'` → **exit 1, 81 errors, 23 warnings**, and **not one of the
errors is a behaviour bug in the unit's text**:

* **33 × `E0428` "defined multiple times", every one with a *mirror* file as the primary span.**
  Twenty-five are `include/msdos.rs` against this unit — slice 1's eight (`dos_set_window_size`,
  `have_mouse`, `mouse_init`, `mouse_on`, `mouse_off`, `mouse_moveto`, `the_only_display_info`,
  `the_only_tty_output`), slice 2's five (`dos_get_saved_screen`, `dos_set_keyboard`,
  `internal_terminal_init`, `initialize_msdos_display`, `IT_set_frame_parameters`) and slice 3's
  twelve (`dos_keysns`, `dos_keyread`, `XMenuCreate`, `XMenuAddPane`, `XMenuAddSelection`,
  `XMenuLocate`, `XMenuActivate`, `XMenuDestroy`, `dostounix_filename`, `unixtodos_filename`,
  `getdefdir`, `rootrelativepath`) — C's "prototype in `msdos.h`, definition in `msdos.c`" pair,
  which Rust spells only if the declaration sits in a glob-imported module (PORTING.md 0.1-10's
  `syssignal_decls` shape; Owed 2).  The other eight are that mirror's *overrides* against the
  port's system mirrors (Owed 2).
* **47 × `E0425` "cannot find", and every one names a *mirror* gap**: **28 distinct names**.  The
  twenty-one slices 1-2 reported are unchanged (`clock_t` C 422, `make_formatted_string` C 555,
  `cursor_to` C 1016, `clear_mouse_face` C 1276, `clock`/`CLOCKS_PER_SEC` C 1505, and the fifteen
  of Owed 5's list); slice 3 adds seven: `lookup_derived_face` (six call sites, C 3089-3098;
  `dispextern.h` 3816), `message_with_string` (C 3265; `lisp.h` 4397), `discard_mouse_events`
  (C 3275; `lisp.h` 5090), `gen_help_event` (C 2707; `keyboard.h` 513), `clear_input_pending`
  (C 3278; `keyboard.h` 490), `kbd_buffer_events_waiting` (C 3277; `keyboard.h` 517) and `strcat`
  (C 3465-3466; `<string.h>`'s own, missing from `lib/include/string.rs`, which declares
  `strcpy`/`strcmp`/`strncmp`/`strlen` but not this one — `src/callproc.rs` 175,
  `src/pdumper.rs` 135 and `src/xdisp.rs` 711 declare it at their crate roots) — Owed 5/12.
* **1 × `E0609`** `no field tty_reverse_p on type face` (C 835, slice 1's; Owed 5).

So the unit's own text contributes **0 errors** to that reading *beyond those mirror gaps*, and
the 33 `E0428`s cannot be decided from this file: the mirror graph has never been compiled with
`msdos` on.  **Four repairs the transcription needed in this batch, all inside slice 3's own
text**: the mirror's `SET_CHAR_GLYPH!` takes `padding_p: bool`, so the six `BUILD_CHAR_GLYPH!`
call sites pass `false` (C writes `0`); C's `*p != ' '` comparison needs `as c_char`; C's
`regs.x.flags` (C 2442) needed the `flags` member slice 1's `<dos.h>` excerpt had left out; and
two `recent_doskeys_index++` statements are `+= 1` (see ①).  **Five additions to the earlier
slices' declaration sections** (the names slice 3 newly uses): `<dos.h>`'s `delay` (C 2736),
`<dir.h>`'s `getdisk`/`_fixpath` (C 3377/3360), `<dpmi.h>`'s `__dpmi_yield` (C 3225), `<pc.h>`'s
`ScreenUpdate` (C 3183) and `<sys/param.h>`'s `MAXPATHLEN` (C 3457) — every one UNVERIFIED
(Deviations 14/26).

**③ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/msdos.c` reads
**`src/msdos.c: 27 conditions, 0 unknown, 0 disagreements [VERIFIED]`** — every one of the file's
27 conditions is decided by the frozen table *and* agrees with clang.  Sixteen are inside the
ported span (slice 1's C 34/62/95/99/109/111/119/402/480/1171, slice 2's C 1784/1796/1907 and
slice 3's C 2433/2465/2787); both arms of each are carried in the port and each row of the
`## Gate mapping` table names the reading it came from.  The per-condition answer for this file is
*"the false arm, for all 27"*, because C 34 is the outermost condition — the reading was taken
with the evaluator's own `dead_branch_map` (`-1` meaning "the frozen configuration compiles the
else/dead side"), not from memory.

**④ `extract-abi.py --check` (vacuous here, and said so).**
`python3 tools/extract-abi.py --check emacs/src/msdos.c femacs/src/msdos.rs` → `OK
emacs/src/msdos.c -> femacs/src/msdos.rs`, exit 0.  This is P-86 ⑩'s empty checklist (the
reference's own live-item lists are all zero, Evidence ①), *not* a comparison of the functions
and the subrs the port carries.

**⑤ The other gates.**
`python3 tools/check-declarations.py` → exit 0 (`10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures`).
`python3 tools/check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 750 external, 35
allowed, 0 stale` — slice 3's new citations (the `<dos.h>`/`<dir.h>`/`<dpmi.h>`/`<pc.h>`/
`<sys/param.h>` sections and the two file-local macros) add no drift, and the integrator's
`frame.h` 1548 → 1273 correction (3.466) is untouched.
`python3 tools/check-ledger-ticks.py` → `234 ledger(s) bound …, 0 failure(s)` (this ledger ticks
nothing: its checklist is empty, so it is not bound).
`python3 tools/check-mirror-dupes.py` → `43 duplicate declaration(s), 19 ratchet row(s), 24
failure(s)` **on this working tree, and none of the 24 is this file's**: every `FAIL` names another
lane's *untracked* `femacs/src/include/gtk.rs` (`?? femacs/src/include/gtk.rs`, the P-90 mirror
round in flight) against `femacs/src/image.rs`/`femacs/src/xwidget.rs`.  `grep msdos` over that
reading is empty, and the 19 ratchet rows are the pre-existing set — i.e. this slice adds no
duplicate.
`python3 tools/check-port-lines.py` → `67 file(s) under ten percent, 87,294 reference line(s); 68
ratchet row(s), 0 new, 1 stale`, and the stale row is `src/pgtkterm.h` (another lane's in-flight
mirror).  `src/msdos.c` is not in the list — this port carries 4562 lines against the reference's
4,350 (**104.9%**; the port's own count exceeds C's because every item carries a `#[cfg]` line and
the two big tables are one element per line).

**⑥ The range table (P-86 ②'s machine gate).**
`python3 tools/check-dead-file-coverage.py` → `info docs/ports/msdos.md: 93 range row(s) cover
src/msdos.c (4350 lines; 0 blank line(s) between blocks need no row)` and
`45 ledger(s) bound (a C range table), 0 failure(s)` — every non-blank line of C 1-4350 is inside
a row, the rows ascend without overlapping, and the last one reaches C 4350.  (The same ledger is
also the one the msdos.h *mirror* resolves to, and the tool reads that pair too: `156 lines`.)

## Deviations

1. **The `gettime`/`settime` rename pair (C 43-47) has no counterpart.**  C's `#define gettime
   dos_h_gettime_`/`#undef gettime` moves `<dos.h>`'s prototypes out of gnulib's way.  The port
   declares neither `<dos.h>`'s `gettime` nor the renamed spelling, so the only `gettime` in this
   unit's scope is gnulib's, through C 69's `systime.h` chain (`lib/include/timespec.rs` 91) — the
   rename and its `#undef` are dropped, not translated.
2. **`#pragma pack(0)` (C 52) is dropped.**  Rust has no `#pragma`; the pragma exists so
   `<dir.h>`'s `pack(4)` does not leak onto the following declarations, and `<dir.h>`'s records
   are not declared here (C 51's `getdisk` is past this slice).
3. **`#undef opendir` (C 53) is dropped.**  C undoes `<dir.h>`'s `opendir` macro before
   `<dirent.h>`; the port never spells `opendir` as a macro, so there is nothing to undo.
4. **The DJGPP stand-in sections are excerpts, and every value in them is UNVERIFIED (P-88 ①).**
   `<dos.h>`'s `union REGS` carries only the members this slice reads or writes
   (`x.ax/bx/cx/dx/es/di`, `h.ah/al/bl/dl`), not the header's full set; `__dpmi_regs` likewise;
   `_CRT0_FLAG_UNIX_SBRK`/`_CRT0_FLAG_FILL_SBRK_MEMORY` are **placeholders** (`= 0`), not
   readings, and the object they feed (`_crt0_startup_flags`) is `#[unsafe(no_mangle)]` because C
   defines it in the file; `dosmemput`'s, `int86`'s, `intdos`'s, `__dpmi_int`'s and the `Screen*`
   prototypes follow this file's own call sites.  No DJGPP header exists here (see the file note),
   so no layout assertion is written for any of them — they are owed (Owed 6).
5. **`alloca` (C 1050) is spelled with the port's stack pool.**  C's `spaces = sp = alloca (i)`
   becomes `sp = sa_stack_alloc (i as usize)` then `spaces = sp` (PORTING.md P-05's reading, the
   `src/coding.rs` 8842 shape); C does not account this request against `sa_avail` and neither
   does the port.
6. **C's assignment-expressions in argument positions are hoisted to statements**, keeping the
   evaluation order C guarantees: C 224-225's `mouse_last_x = regs.x.cx = x * 8` is two statements
   (the assignment operator's value after truncation is kept, `regs.x.cx` first); C 329-331's
   `make_fixnum (mouse_last_x = ix)` and `make_fixnum (mouse_last_y = iy)` are each a store then a
   call; C 1216's `ScreenSetCursor (current_pos_Y = new_pos_Y, current_pos_X = new_pos_X)` is two
   stores then the call (C leaves the two argument orders unspecified; the stores happen first
   either way).
7. **The `frame` bit-field accessor is defined by this unit.**  `include/frame.rs` carries
   `FRAME_MOUSE_MOVED_SHIFT`/`_UNIT` but no reader/writer, and the tree defines them per consuming
   unit (`src/window.rs` 152's `set_frame_flag1!`, `src/androidterm.rs` 235's `mouse_moved!`);
   this unit's `mouse_moved!`/`set_frame_flag1!` copy is under the file gate.  A unit that needs
   two of these macros must define both locally — Owed 7 asks the mirror round to settle it.
8. **`coding->common_flags` and `coding->mode` are read and written through the mirror's own
   shift/mask constants** (`CODING_SYSTEM_COMMON_FLAGS_SHIFT`, `CODING_SYSTEM_MODE_SHIFT`,
   `CODING_MODE_LAST_BLOCK`): `include/coding.rs` 235-244 keeps them as bit fields of
   `coding_system.bits` and says so ("the port masks"), and it has no public `common_flags`/`mode`
   setter (its own `mode_of` is private and it notes "the writers land with coding.c").  C's
   comment at C 893-895 — "We can't use `CODING_REQUIRE_ENCODING` here because it always returns 1
   if `terminal_coding.src_multibyte` is 1" — is therefore honoured literally: the port masks
   `CODING_REQUIRE_ENCODING_MASK` itself and does **not** call the mirror's helper.
9. **`glyph_row.used[]` is `c_short` in the mirror**, so every read is `as c_int` before it meets
   an `int` (C 994-998, 1001); the cast is the port's, C's promotion is implicit.
10. **The two `use` edges this unit adds** — `use frame_face_accessors::*;` (gated with
    `dispextern_face_records`) and `use dispextern_unit_names::*;` — exist because those mirrors
    declare the modules and never glob-import them (Owed 5/6).
11. **`IT_cmgoto`'s `#if 0` region (C 1171-1211) is carried as a comment**, not as Rust text: C's
    own preprocessor never compiles it, and the port keeps the reference's characters visible (the
    `src/cm.rs` 86 precedent) with no `#[cfg]` that could ever be turned on.
12. **`countof (std_dimension)` carries `as c_int`** (C 953) — C's `int < size_t` comparison has
    no implicit promotion in Rust, and the mirror's `countof!` yields `usize`.
13. **The include edges the C file reaches only through `lisp.h` are written at C 36.**
    `<stdcountof.h>` (C's own `countof`, needed at C 953) is `include/lisp.rs`'s chain in C but is
    a separate edge in this tree (`src/abi_shim.rs` 26, `src/alloc.rs` 64 spell it the same way),
    so the `include!` sits under C 36's `#include "lisp.h"` line.

**Slice 2's deviations** (numbers continue the list above):

14. **The two DJGPP stand-in sections grew for slice 2, and every new item is UNVERIFIED
    (P-88 ①).**  The C 57 `<dpmi.h>` section gains `_go32_dpmi_registers` (with `Go32DpmiRegsX`/
    `Go32DpmiRegsH`) and `_go32_dpmi_simulate_int` (C 2055/2062), and the C 87 `<pc.h>` section
    gains `ScreenMode` (C 1436) and `ScreenRetrieve` (C 1457).  No DJGPP header exists here, so
    the record's size, its two views and the two prototypes follow this file's own call sites;
    the ledger's Owed list names the missing toolchain (Owed 8).
15. **The keyboard maps' non-ASCII bytes (C 1972-2011) are spelled `\xNN`.**  The reference is
    codepage 850 and the port's own text stays ASCII, so each of the eight lines carries C's
    bytes as escapes (e.g. C 1972's `"\xfd&\x82\"'(-\x8a_\x80\x85)=  azertyuiop^$…"`); the
    decoded length of every line matches C's (57 bytes, and 58 for C 2011).
16. **`alloca` (C 1569-1572) is spelled with the port's stack pool**, the C 1050 reading again
    (Deviations 5): C's two `alloca (length * word_size)` requests become two `sa_stack_alloc`
    calls, and C accounts neither against `sa_avail` nor does the port.
17. **The per-unit helpers slice 2 needs are defined in this unit.**  `hook!` (the `src/term.rs`
    140-144 transmute for a Rust-ABI function stored in an `Option<unsafe extern "C" fn …>`
    field), `output_method!` (frame.h 446's four-bit field, the `src/dispnew.rs` 7349 /
    `src/frame.rs` 342 shape), `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL` (frame.h
    1375-1376's field macros, the `src/frame.rs` 134/137 shape),
    `FRAME_NO_SPECIAL_GLYPHS` (frame.h 1273's writer arm) and `default_face_done_p!` (frame.h
    340's flag, the unit's own `mouse_moved!` shape).  Every one carries the file gate.
18. **The two file-scope anonymous structs are named.**  C 2082-2088 and C 2102-2106 declare
    anonymous `struct { … } keypad_translate_map[]`/`grey_key_translate_map[]`; the port names
    their element types `keypad_translate_map_entry` and `grey_key_translate_map_entry`
    (PORTING.md 1.3, the `src/charset.rs` 386 `charset_map_entry` shape).
19. **`struct dos_keyboard_map`'s three `char *` slots are `*const c_char`.**  Each only ever
    holds a string literal, which is how this tree spells that C field (`include/termchar.rs`
    178's `TS_orig_pair`).
20. **C's implicit numeric conversions inside the tables are spelled.**  `ibmpc_translate_map`'s
    element type is C's `unsigned short`, so each `int` combination carries `as c_ushort` (167 of
    them, C 2118-2304); the two translate tables' `code` fields likewise; `countof
    (keyboard_layout_list)` carries `as c_int` (C 2071, the C 953 reading again).
21. **Chained assignments are written as their stores**, keeping C's order: C 1414-1415
    (`new_pos_X = new_pos_Y = 0`, C 1415's `current_pos_X = current_pos_Y = -1`), C 1457
    (`ScreenRetrieve (startup_screen_buffer = xmalloc (…))`), C 1829
    (`initial_screen_colors[0] = initial_screen_colors[1] = -1`), C 1877
    (`cursor_to_hook = raw_cursor_to_hook = IT_cursor_to`) and C 2061
    (`regs.x.ss = regs.x.sp = regs.x.flags = 0`).
22. **C 1769's `extern void init_frame_faces (struct frame *);` is dropped** — it re-declares
    `dispextern.h` 3821's prototype, and the port does not repeat a declaration the mirror owns
    (PORTING.md 0.1-12).  C 1771's `#endif` is carried as the comment that closes the C 480 block:
    every item after it (`internal_terminal_init` onward) is gated by the file alone.
23. **C 1917's and C 2306's form-feed lines carry no port line.**  They are the reference's page
    separators, the coverage gate's blank-line rule leaves them out (Evidence ⑥), and two of the
    reference's neighbours (`src/keyboard.rs` 5470-5477, `src/xdisp.rs`) show the other reading.
24. **Two include edges are written at their C include lines** (the C 36 reading, Deviations 13):
    `include!("../lib/include/string.rs")` under C 36 (lisp.h 30's `#include <string.h>`, for
    C 1794's `strcmp`) and `include!("../lib/include/stdlib.rs")` under C 50 (gnulib's
    `lib/unistd.in.h` 122-125 includes `<stdlib.h>` for every non-glibc target, for C 1780's
    `getenv`).  Neither mirror carries its own edge (Owed 10).
25. **Two repairs outside slice 2's new text**, both forced by committed mirror changes and both
    in this file: `IT_write_glyphs`'s `FRAME_TERMINAL_CODING! (f) as *mut coding_system` (commit
    `56f55bb`, status 3.455, whose message lists the sixteen sites it fixed in five other units
    and missed this one) and `x_set_title (f as *mut c_void, val)` (the `dosfns.h` mirror's
    parameter type).  Evidence ② records both readings.

**Slice 3's deviations** (numbers continue):

26. **The DJGPP stand-in sections grew a third time, and every new item is UNVERIFIED
    (P-88 ①).**  The C 45 `<dos.h>` section gains `delay` (C 2736 — a DJGPP runtime function with
    no declaration anywhere in the reference tree), C 51's `<dir.h>` section is *new* (it was a
    comment saying `getdisk` belonged to a later slice, which slice 3 falsifies): `getdisk`
    (C 3377) and `_fixpath` (C 3360); C 57's `<dpmi.h>` gains `__dpmi_yield` (C 3225) and C 87's
    `<pc.h>` gains `ScreenUpdate` (C 3183).  No DJGPP header exists here, so every prototype
    follows this file's own call sites and the ledger's Owed 8/12 names the missing toolchain.
27. **C 39's `<sys/param.h>` now has a one-name stand-in section.**  Slice 1's comment said "no
    name this slice takes comes from it"; C 3457's `emacsroot` and C 3462's `result` are its
    `MAXPATHLEN`, so the constant is declared at that include line with the tree's measured value
    (`src/sysdep.rs` 369-370, `src/pdumper.rs` 218-219) and `usize`'s type because both uses are
    array sizes.
28. **The `input_event_bits!` and `mouse_face_hidden!` per-unit helpers.**  `input_event`'s
    `kind : 16` shares the mirror's `bits` word with `part : 16` (`include/termhooks.rs` 100-117),
    so C's `event.kind = x;` is a per-unit macro — the `src/androidterm.rs` 201-210 /
    `src/androidselect.rs` 136-143 shape, body-identical.  `Mouse_HLInfo`'s `mouse_face_hidden`
    (dispextern.h 3023) has no accessor pair in the mirror (its own 2084-2095 note records that
    `docs/ports/w32term.md` owed 5 asks for it), so this unit masks `MOUSE_FACE_HIDDEN` itself,
    the way its `mouse_moved!` names the frame flag.
29. **`BUILD_CHAR_GLYPH` keeps C's name as a `macro_rules!`.**  C 2880-2887's file-local macro
    expands to the mirror's `glyph_bits!` (the `type` member) and `SET_CHAR_GLYPH!`, and C's
    `(GLYPH).charpos = -1` is a field store; C's `while (0)` is dropped the way
    `docs/ports/macro-body-notes.tsv` 283 records for `SET_CHAR_GLYPH` itself.  The mirror's
    `padding_p` is a Rust `bool`, so the six call sites pass `false` where C writes `0`.
30. **The fall-through `switch (code & 0xf000)` (C 2543-2635) is a loop with a fall state.**  It
    falls through twice (`ModFct` into `FctKey` into `Special`; `Normal` into `Map`) and contains
    `continue`s that C sends to the enclosing `while`, so the port keeps C's order with a state
    variable (the `src/ccl.rs` 1734 shape), spells those `continue 'dos_rawgetc_keys` with the
    loop label (the `src/bytecode.rs` 835 shape) and lets C's `break` be the inner loop's.  C's
    `if (!keyboard_map_all) { …; break; }` is a `break`, not the fall-through (it exits the
    switch in C).
31. **C's assignment-in-condition forms are hoisted, in C's order** (the slice-2 reading): C
    2496/2512/2516's `(code = …)` tests, C 2770's `(prev_get_char = dos_rawgetc ())`, C 3016's
    `(menu = IT_menu_search_pane (…))`, C 3125/3198's two `ScreenRetrieve (state[…].screen_behind
    = xmalloc (…))`, C 3135's `(onepane = …)`, C 2809/2821/2829's and C 3301's chained stores,
    C 2990's `++menu->panecount` and C 3319/3331/3353-3357/3396/3399/3402's `*p++` stores.
32. **`alloca` (C 3086) is spelled with the port's stack pool**, the C 1050 reading again
    (Deviations 5/16): `state` comes from `sa_stack_alloc` and C accounts nothing against
    `sa_avail`.
33. **C's `register unsigned char c` (C 2445) is a plain local** — C's storage class has no Rust
    counterpart and nothing observable depends on it.
34. **C's uninitialized locals are zero-initialized (PORTING.md 1.5)**: `IT_menu_display`'s `h2`
    and `w2` (written by the two out-pointer calls), `XMenuActivate`'s `title_faces`/`faces`
    arrays (fully written before use) and `getdefdir`'s `in_path` (written before `_fixpath`
    reads it).
35. **C 3307's `void msdos_downcase_filename (unsigned char *);` is dropped** — a forward
    declaration of the definition at C 3417, the C 1769 reading (PORTING.md 0.1-12).
36. **`IT_menu_search_pane`'s local is `r#try`.**  C names it `try`; `try` is a Rust keyword, so
    the port writes the raw identifier, which keeps C's name rather than renaming it (the
    `r#type` reading in `include/termhooks.rs` 204).
37. **Slice 1's `<dos.h>` excerpt gained `RegsX.flags`.**  C 2442 reads `regs.x.flags`, which the
    two-view excerpt (Deviations 4) had not carried; the member joins the same UNVERIFIED record.
38. **The four `dos_rawgetc`/menu pointer reads keep C's indexing**: `keyboard->shifted[code]` and
    its two neighbours become `*(*keyboard).shifted.offset (code as isize) as c_int` (the
    `c_char`→`c_int` sign extension C's assignment performs), and every `state[i]`/`menu->text[i]`
    becomes `(*…offset (i as isize))` so the expressions stay the ones C wrote.
39. **`__dpmi_yield`, `ScreenUpdate`, `getdisk`, `_fixpath`, `delay` and the `MAXPATHLEN` arrays
    are the only new runtime names slice 3 takes**; everything else it needs is either a mirror
    name (Owed 5/10) or one of this unit's own earlier slices.

## Owed (nothing here is claimed as done)

1. **`femacs/xtask/Cargo.toml`**: three feature names this unit uses are registered nowhere —
   `gnu_malloc` (C 111), `_use_lfn` (C 95) and `_dos_ds` (C 99).  (`msdos`, `no_system_malloc`,
   `rel_alloc`, `have_x_windows` and `djgpp_2_3` are precedents that are registered.)
   ✅ **Stale, closed by the integrator in 3.466**: all three *are* registered on this revision —
   `_dos_ds` line 30, `_use_lfn` line 31, `gnu_malloc` line 93 of `femacs/xtask/Cargo.toml`, each
   with its C site in the comment (`_dos_ds (src/msdos.c)`, `_USE_LFN`, `GNU_MALLOC`), and
   `tools/feature-coverage.py` lists all three with their sites.  Nothing to register.
2. **`include/msdos.rs` must move its prototypes into a glob-imported module.**  **Twenty-five**
   `E0428`s come from that mirror's top-level `extern "C"` blocks against this unit's definitions:
   slice 1's eight (`dos_set_window_size`, `have_mouse`, `mouse_init`, `mouse_on`, `mouse_off`,
   `mouse_moveto`, `the_only_display_info`, `the_only_tty_output`), slice 2's five
   (`internal_terminal_init`, `initialize_msdos_display`, `dos_set_keyboard`,
   `dos_get_saved_screen`, `IT_set_frame_parameters`) and slice 3's twelve (`dos_keysns`,
   `dos_keyread`, `XMenuCreate`, `XMenuAddPane`, `XMenuAddSelection`, `XMenuLocate`,
   `XMenuActivate`, `XMenuDestroy`, `dostounix_filename`, `unixtodos_filename`, `getdefdir`,
   `rootrelativepath`).  The shape is `include/syssignal.rs` 29-52
   (`pub mod syssignal_decls` + `use syssignal_decls::*;`) and the rule is PORTING.md 0.1-10.
   Until it lands, **every** `msdos` translation unit that includes `msdos.h` and defines one of
   those names reports `E0428`, and the count grows with every slice this file gains.  The other
   eight `E0428`s of the same reading are that mirror's *overrides* against
   `lib/include/unistd.in.rs` 39-41, `sys_stat.in.rs` 181-196, `errno.rs` 92 and `fcntl.in.rs`
   118 (the `docs/ports/w32proc.md` Owed 5(a) class): the MS-DOS side is a *deliberate
   re-declaration* of the same name, so the reconciliation is the mirror round's (feature-mask
   the MS-DOS copy, as `include/msdos.rs` already does for `EINPROGRESS`/`O_CLOEXEC` at 84-87/80 —
   the pattern exists but the two files' rows have not been made consistent).
3. **`docs/baseline/port-lines-pending.tsv`: closed, and said so.**  Slice 1's row
   (`src/msdos.c 4350 24 435`) is gone — `grep msdos docs/baseline/port-lines-pending.tsv` is
   empty on this revision, and `check-port-lines.py` reads `0 new, 1 stale`, the stale row being
   `src/pgtkterm.h` (another lane's in-flight mirror).  This file carries 4562 lines against the
   reference's 4,350 (104.9%), far above the ten-percent threshold.  Nothing is owed here; the line
   is kept so the next reader knows the row was measured rather than forgotten.
4. **`lib/include/time.in.rs` (or `time.rs`): `clock_t`, `clock` and `CLOCKS_PER_SEC`.**  C 422's
   `static clock_t startup_time;` has no counterpart — no mirror in this tree declares `clock_t`
   at all — and slice 2's C 1505 (`clock () - startup_time >= 2*CLOCKS_PER_SEC`) adds the
   function and the macro to the same gap.  C reaches all three through C 38's `#include <time.h>`,
   whose mirror is `time.in.rs`.  Until they land the `msdos` arm reports three `E0425`s.
5. **`include/dispextern.rs` and its neighbours: twenty-four names the mirrors lack.**  Slice 1's
   five are `cursor_to` (C 1016), `clear_mouse_face` (C 1276), `face.tty_reverse_p` (C 835, the
   reading's one `E0609`) and the pair Owed 6/4 name (`make_formatted_string`, `clock_t`); slice 2
   adds, each with the reference line that declares it: `update_face_from_frame_parameter`
   (`dispextern.h` 3794), `load_color` (3804), `init_frame_faces` (3821), `tty_color_name` (3801),
   `tty_defined_color` (3796), `change_frame_size` (3924), `redraw_frame` (3896),
   `is_tty_child_frame` (4013), `note_mouse_highlight` (3688 — which `include/dispextern.rs`
   itself records as *owed, not declared* at 3913, because four other units already declare it at
   their crate roots), `tty_child_size_param` (`frame.h` 1483), `tty_child_pos_param` (1481),
   `root_frame` (1508), `store_frame_param` (`lisp.h` 5117), `tty_read_avail_input`
   (`keyboard.h` 520) and `x_menu_show` (`menu.h` 48).  `x_menu_show` is its own case: the
   declaration is live for MSDOS (`#if defined (HAVE_X_WINDOWS) || defined (MSDOS)`) while the
   mirror keeps only the frozen configuration's dead text, so it is an **arm**, not a missing row
   — the `docs/ports/menu.md` round decides the gate.  **Slice 3 adds six**, each with the
   reference line that declares it: `lookup_derived_face` (six call sites, C 3089-3098;
   `dispextern.h` 3816), `message_with_string` (C 3265; `lisp.h` 4397), `discard_mouse_events`
   (C 3275; `lisp.h` 5090), `gen_help_event` (C 2707; `keyboard.h` 513), `clear_input_pending`
   (C 3278; `keyboard.h` 490) and `kbd_buffer_events_waiting` (C 3277; `keyboard.h` 517) — the
   last five are all *defined* in `keyboard.rs`/`lisp.h`'s owner units, which is why
   `src/dispnew.rs` 279-281 declares `gen_help_event` at its own crate root, the alternative this
   lane does not take (its slices 1-2 record the same choice).
6. **`make_formatted_string` (`lisp.h`) is missing from `include/lisp.rs`** — C 555's only use.
7. **The mirror round's reconciliation of the `dispextern_unit_names`/`frame_face_accessors`
   edges.**  Both modules are `pub mod`s inside a mirror that never glob-imports them
   (`include/syssignal.rs` 52 is the shape they lack), so *this* unit adds its own `use` edges
   (Deviations 10).  Either the mirrors add the glob or the per-consumer edges become the
   convention; the decision is the mirror round's, and until it is made every consumer names them
   itself.
8. **The layout assertions for the DJGPP records and prototypes this unit spells** (`REGS`,
   `RegsX` with slice 3's `flags`, `RegsH`, `__dpmi_regs`, `_go32_info_block`, slice 2's
   `_go32_dpmi_registers` with its two views, and slice 3's `getdisk`/`_fixpath`/`__dpmi_yield`/
   `ScreenUpdate`/`delay`): `xtask layout-asserts` reads a C compiler's readings, and no DJGPP
   toolchain exists here, so not one `core::assert!` is written for them and `_CRT0_FLAG_*`'s two
   constants stay `= 0` placeholders (Deviations 4/14/26).
9. **The next lane's slice: C 3469-4350**, whose first live definition is C 3475-3654's
   `init_environment` (C 3470-3473 is its banner).  Its own conditions are C 3565 and C 3671
   (`#ifdef HAVE_X_WINDOWS`), C 3691 and C 3716 (`#ifndef HAVE_X_WINDOWS`), C 3895 and C 4135
   (`__DJGPP__ == 2 && __DJGPP_MINOR__ < 4`), C 4128 (`__DJGPP__ > 2 || __DJGPP_MINOR__ > 3`),
   C 4158 (`!HAVE_SELECT`), C 4252 (`#ifdef chdir`), C 4281 (`#ifdef tzset`) and C 4326
   (`#ifndef HAVE_X_WINDOWS`) — eleven of the file's 27, and `init_environment` is the file's
   other `alloca` user (C 3538, 3574, 3586 and more), so the C 1050 reading applies again.  The
   file's only non-ASCII text (C 1972-2011) is behind this ledger.
10. **The two include edges this unit writes for itself** (Deviations 24): `string.rs` under C 36
    and `stdlib.rs` under C 50.  Both are edges C reaches through gnulib headers (`lisp.h` 30's
    `<string.h>`, `lib/unistd.in.h` 122-125's `<stdlib.h>`) that the mirrors do not carry, so the
    mirror round either adds them to `include/lisp.rs`/`lib/include/unistd.in.rs` or the
    per-consumer spelling becomes the convention (the same question Deviations 13 raised).
11. **Status 3.455's coding_system mirror round missed this file.**  Commit `56f55bb` cast
    `FRAME_TERMINAL_CODING!` at its sixteen call sites in `coding.rs`/`terminal.rs`/`composite.rs`/
    `keyboard.rs`/`term.rs` and not in `msdos.rs`, whose `IT_write_glyphs` is the one consumer
    left (Deviations 25).  The repair is in the second slice's batch; the round's own audit is what
    should say whether any other file was missed.
12. **`lib/include/string.rs` lacks `strcat`.**  It declares `strlen`/`strcmp`/`strncmp`/`strcpy`
    (28-34) but not `strcat`, which C 3465-3466 needs; `src/callproc.rs` 175, `src/pdumper.rs`
    135 and `src/xdisp.rs` 711 each declare it at their own crate root, and this lane's Owed 5
    policy (leave it to the mirror round, record the error) is why the `msdos` arm reports one
    more `E0425`.  The mirror round is the fix either way, and it should decide whether the three
    use-site copies move with it (the P-90 ① "one surface, one mirror" question).

## Not claimed

Every line of all three slices sits behind gates the frozen configuration leaves false: Evidence
① is an empty object, and the readings of ②-⑥ are compile-level or textual.  **Nothing here was
executed** — no `int86`/`intdos`/`__dpmi_int`/`_go32_dpmi_simulate_int`/`__dpmi_yield`, no BIOS
call, no far-pointer store, no mouse, no screen write, no keyboard map, no menu and no filename
conversion was ever run.  There is **no DJGPP-target compile and no layout measurement**: the arm
flip is rustc on this machine with two `--cfg`s, and its 81 errors are **all** in the mirror graph
or the 28 names the mirrors lack — the unit's own text contributes none of its own (Evidence ②).
The file's `#if` verdicts are `tools/check-cond-eval.py`'s (and it is `[VERIFIED]` for this file),
not clang's.  C 3469-4350 is **not ported here**; the vacuously-passing `extract-abi.py --check`,
the 28 missing names of Evidence ②, the 12 owed items and the 20.3% of the file left are that
boundary, not defects.  The `msdos` arm does not compile today, and that is the mirror graph's
state, not these slices'.
