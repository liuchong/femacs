# emacs/src/msdos.c (4,350 lines) — slices 1-2: C 1-2422

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
> (**3209 lines**, 127,437 bytes, md5 `cfb570ab7922797e9400af6b9f28b804`), carrying **C 1-2422**
> with **both arms of every `#if`** (slice 1: C 1-1295; slice 2: C 1296-2422).
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
> **The next live definition is C 2425-2759's `dos_rawgetc`** (C 2423 is its blank line and C 2424
> its banner), and the remainder (C 2423-4350) is the table's last row: `dos_rawgetc`,
> `dos_keysns`/`dos_keyread`, the menu simulation, the DOS/V screen buffer, the file-name layer
> (`unixtodos_filename` and friends), the subprocess layer and `syms_of_msdos`, none of it ported
> here.
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

**Thirteen of the file's 27 conditions are inside the ported span** — C 34, C 62, C 95, C 99,
C 109, C 111, C 119, C 402, C 480 and C 1171 (slice 1's ten) plus slice 2's C 1784, C 1796 and
C 1907; the remaining fourteen are past C 2422 and each is named in Owed 9.  C 1171's `#if 0` is
the one with no arm at all (see its row).

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
| 2423-4350 | the unported remainder (C 2424-4350; `dos_rawgetc` onward) | not ported this batch |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos2/cargo cargo run -q -p xtask -- build-tu src/msdos.c`
→ `built src/msdos.rs -> ../build/femacs/msdos.o`, **exit 0**, and `build/femacs/msdos.o` is
**336 bytes** (sha256 `d1ea88b2fe3d960975d134e566e6593217e9ad67abdc2db5a451ea9620edd0ed` — the same
reading slice 1 took, which is what P-86 ⑧ predicts).  Every item of C 1-2422 sits behind a `msdos`
gate, so under the frozen feature set rustc compiles only `include/config.rs` and the `#![allow]`
attribute; an `include!` outside the gate would break this compile at once.

**② The `msdos` arm flip, as a per-file error census (P-86 ③⑴/⑦).**  There is no DJGPP target to
compile for, so this is rustc on *this* machine with the derived feature set:
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos2/cargo cargo run -q -p xtask -- probe src/msdos.rs
/tmp/msdos_arm9.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"' --cfg
'feature="dispextern_face_records"'` → **exit 1, 53 errors, 21 warnings**, and **not one of the
errors is a behaviour bug in the unit's text**:

* **21 × `E0428` "defined multiple times", every one with a *mirror* file as the primary span.**
  Thirteen are `include/msdos.rs` against this unit: slice 1's eight (`dos_set_window_size` 88,
  `have_mouse` 116, `mouse_init` 117, `mouse_on` 118, `mouse_off` 119, `mouse_moveto` 120,
  `the_only_display_info` 250, `the_only_tty_output` 251) and slice 2's five
  (`dos_get_saved_screen` 85, `dos_set_keyboard` 87, `internal_terminal_init` 105,
  `initialize_msdos_display` 106, `IT_set_frame_parameters` 126) — C's "prototype in `msdos.h`,
  definition in `msdos.c`" pair, which Rust spells only if the declaration sits in a
  glob-imported module (PORTING.md 0.1-10's `syssignal_decls` shape; Owed 2).  The other eight are
  that mirror's *overrides* against the port's system mirrors —
  `readlinkat`/`fstatat`/`faccessat`/`fchmodat`/`futimens`/`utimensat` (`lib/include/unistd.in.rs`
  39-41, `lib/include/sys_stat.in.rs` 181-196), `EINPROGRESS` (`lib/include/errno.rs` 92) and
  `O_CLOEXEC` (`lib/include/fcntl.in.rs` 118) — the `docs/ports/w32proc.md` Owed 5(a) class
  (Owed 2).
* **31 × `E0425` "cannot find", and every one names a *mirror* gap**: 22 distinct names.  Five are
  slice 1's (`clock_t` C 422, `make_formatted_string` C 555, `cursor_to` C 1016,
  `clear_mouse_face` C 1276, and `face.tty_reverse_p` C 835 — the last is the reading's one
  `E0609`); slice 2 adds seventeen: `update_face_from_frame_parameter` (six call sites,
  C 1641-1755; `dispextern.h` 3794), `load_color` (C 1629/1659; `dispextern.h` 3804),
  `init_frame_faces` (C 1586/1867; `dispextern.h` 3821), `tty_color_name` (C 1749/1756;
  `dispextern.h` 3801), `tty_defined_color` (C 1901; `dispextern.h` 3796), `change_frame_size`
  (C 1721; `dispextern.h` 3924), `redraw_frame` (C 1765; `dispextern.h` 3896),
  `is_tty_child_frame` (C 1716; `dispextern.h` 4013), `note_mouse_highlight` (C 1303, through
  `include/frame.rs` 1658's `FRAME_MOUSE_UPDATE!`; `dispextern.h` 3688, which that mirror's own
  note at 3913 records as **owed, not declared**), `tty_child_size_param` (C 1718/1719;
  `frame.h` 1483), `tty_child_pos_param` (C 1723/1724; `frame.h` 1481), `root_frame` (C 1729;
  `frame.h` 1508), `store_frame_param` (C 1711; `lisp.h` 5117), `tty_read_avail_input` (C 1900;
  `keyboard.h` 520), `x_menu_show` (C 1893; `menu.h` 48, inside
  `#if defined (HAVE_X_WINDOWS) || defined (MSDOS)` — a declaration the mirror drops with the
  frozen configuration's dead text), and `<time.h>`'s `clock` and `CLOCKS_PER_SEC` (both C 1505;
  `lib/include/time.in.rs`) — Owed 4/5.

So the unit's own text contributes **0 errors** to that reading *beyond those mirror gaps*, and
the 21 `E0428`s cannot be decided from this file: the mirror graph has never been compiled with
`msdos` on.  **Two repairs this batch made, both outside slice 2's new text and both named**:
(a) `IT_write_glyphs`'s `FRAME_TERMINAL_CODING! (f)` needed the `as *mut coding_system` cast that
commit `56f55bb` (status 3.455's coding_system mirror round) added to its other sixteen consumers
and missed in this file; (b) `x_set_title (f as *mut c_void, val)` spells the parameter the
`dosfns.h` mirror declares (P-72 rule ③).  Before them the same command read **57 errors, 21
warnings** (the two extra were an `E0308` at each site).

**③ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/msdos.c` reads
**`src/msdos.c: 27 conditions, 0 unknown, 0 disagreements [VERIFIED]`** — every one of the file's
27 conditions is decided by the frozen table *and* agrees with clang.  Thirteen are inside the
ported span (slice 1's C 34/62/95/99/109/111/119/402/480/1171 and slice 2's C 1784/1796/1907);
both arms of each are carried in the port and each row of the `## Gate mapping` table names the
reading it came from.  The per-condition answer for this file is *"the false arm, for all 27"*,
because C 34 is the outermost condition — the reading was taken with the evaluator's own
`dead_branch_map` (`-1` meaning "the frozen configuration compiles the else/dead side"), not from
memory.

**④ `extract-abi.py --check` (vacuous here, and said so).**
`python3 tools/extract-abi.py --check emacs/src/msdos.c femacs/src/msdos.rs` → `OK
emacs/src/msdos.c -> femacs/src/msdos.rs`, exit 0.  This is P-86 ⑩'s empty checklist (the
reference's own live-item lists are all zero, Evidence ①), *not* a comparison of the functions
and the subrs the port carries.

**⑤ The other gates.**
`python3 tools/check-declarations.py` → exit 0 (`10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures`).
`python3 tools/check-mirror-dupes.py` → `19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)`.
`python3 tools/check-ledger-ticks.py` → `234 ledger(s) bound …, 0 failure(s)` (this ledger ticks
nothing: its checklist is empty, so it is not bound).
`python3 tools/check-port-lines.py` → `69 file(s) under ten percent …, 0 new, 0 stale`;
`src/msdos.c` is not among them — this port carries 3209/4350 = **73.8%** of the reference's
lines.

**⑥ The range table (P-86 ②'s machine gate).**
`python3 tools/check-dead-file-coverage.py` → `info docs/ports/msdos.md: 68 range row(s) cover
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

## Owed (nothing here is claimed as done)

1. **`femacs/xtask/Cargo.toml`**: three feature names this unit uses are registered nowhere —
   `gnu_malloc` (C 111), `_use_lfn` (C 95) and `_dos_ds` (C 99).  (`msdos`, `no_system_malloc`,
   `rel_alloc`, `have_x_windows` and `djgpp_2_3` are precedents that are registered.)
   ✅ **Stale, closed by the integrator in 3.466**: all three *are* registered on this revision —
   `_dos_ds` line 30, `_use_lfn` line 31, `gnu_malloc` line 93 of `femacs/xtask/Cargo.toml`, each
   with its C site in the comment (`_dos_ds (src/msdos.c)`, `_USE_LFN`, `GNU_MALLOC`), and
   `tools/feature-coverage.py` lists all three with their sites.  Nothing to register.
2. **`include/msdos.rs` must move its prototypes into a glob-imported module.**  **Thirteen**
   `E0428`s come from that mirror's top-level `extern "C"` blocks against this unit's definitions:
   slice 1's eight (`dos_set_window_size`, `have_mouse`, `mouse_init`, `mouse_on`, `mouse_off`,
   `mouse_moveto`, `the_only_display_info`, `the_only_tty_output`) and slice 2's five
   (`internal_terminal_init`, `initialize_msdos_display`, `dos_set_keyboard`,
   `dos_get_saved_screen`, `IT_set_frame_parameters`).  The shape is `include/syssignal.rs` 29-52
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
   empty on this revision and `check-port-lines.py` reads `0 new, 0 stale`, with the port at
   3209/4350 = 73.8%, above the ten-percent threshold.  Nothing is owed here; the line is kept so
   the next reader knows the row was measured rather than forgotten.
4. **`lib/include/time.in.rs` (or `time.rs`): `clock_t`, `clock` and `CLOCKS_PER_SEC`.**  C 422's
   `static clock_t startup_time;` has no counterpart — no mirror in this tree declares `clock_t`
   at all — and slice 2's C 1505 (`clock () - startup_time >= 2*CLOCKS_PER_SEC`) adds the
   function and the macro to the same gap.  C reaches all three through C 38's `#include <time.h>`,
   whose mirror is `time.in.rs`.  Until they land the `msdos` arm reports three `E0425`s.
5. **`include/dispextern.rs` and its neighbours: eighteen names the mirrors lack.**  Slice 1's
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
   — the `docs/ports/menu.md` round decides the gate.
6. **`make_formatted_string` (`lisp.h`) is missing from `include/lisp.rs`** — C 555's only use.
7. **The mirror round's reconciliation of the `dispextern_unit_names`/`frame_face_accessors`
   edges.**  Both modules are `pub mod`s inside a mirror that never glob-imports them
   (`include/syssignal.rs` 52 is the shape they lack), so *this* unit adds its own `use` edges
   (Deviations 10).  Either the mirrors add the glob or the per-consumer edges become the
   convention; the decision is the mirror round's, and until it is made every consumer names them
   itself.
8. **The layout assertions for the DJGPP records this unit spells** (`REGS`, `RegsX`, `RegsH`,
   `__dpmi_regs`, `_go32_info_block`, and slice 2's `_go32_dpmi_registers` with its two views) and
   the values of `_CRT0_FLAG_*`: `xtask layout-asserts` reads a C compiler's readings, and no
   DJGPP toolchain exists here, so not one `core::assert!` is written for them and the two flag
   constants stay `= 0` placeholders (Deviations 4/14).
9. **The next lane's slice: C 2423-4350**, whose first live definition is C 2425-2759's
   `dos_rawgetc` (C 2424 is its banner).  Its own conditions are C 2433 and C 2465
   (`#ifndef HAVE_X_WINDOWS`, both inside `dos_rawgetc`), C 2787 (`#ifndef HAVE_X_WINDOWS`), C 3565
   and C 3671 (`#ifdef HAVE_X_WINDOWS`), C 3691 and C 3716 (`#ifndef HAVE_X_WINDOWS`), C 3895 and
   C 4135 (`__DJGPP__ == 2 && __DJGPP_MINOR__ < 4`), C 4128 (`__DJGPP__ > 2 || __DJGPP_MINOR__ > 3`),
   C 4158 (`!HAVE_SELECT`), C 4252 (`#ifdef chdir`), C 4281 (`#ifdef tzset`) and C 4326
   (`#ifndef HAVE_X_WINDOWS`) — fourteen of the file's 27, and **the file's only non-ASCII text
   (C 1972-2011) is now behind this ledger** rather than in front of it.
10. **The two include edges this unit writes for itself** (Deviations 24): `string.rs` under C 36
    and `stdlib.rs` under C 50.  Both are edges C reaches through gnulib headers (`lisp.h` 30's
    `<string.h>`, `lib/unistd.in.h` 122-125's `<stdlib.h>`) that the mirrors do not carry, so the
    mirror round either adds them to `include/lisp.rs`/`lib/include/unistd.in.rs` or the
    per-consumer spelling becomes the convention (the same question Deviations 13 raised).
11. **Status 3.455's coding_system mirror round missed this file.**  Commit `56f55bb` cast
    `FRAME_TERMINAL_CODING!` at its sixteen call sites in `coding.rs`/`terminal.rs`/`composite.rs`/
    `keyboard.rs`/`term.rs` and not in `msdos.rs`, whose `IT_write_glyphs` is the one consumer
    left (Deviations 25).  The repair is in this batch; the round's own audit is what should say
    whether any other file was missed.

## Not claimed

Every line of both slices sits behind gates the frozen configuration leaves false: Evidence ① is
an empty object, and the readings of ②-⑥ are compile-level or textual.  **Nothing here was
executed** — no `int86`/`intdos`/`__dpmi_int`/`_go32_dpmi_simulate_int`, no BIOS call, no
far-pointer store, no mouse, no screen write, no keyboard map was ever consulted and no `IT_*`
entry point was ever called.  There is **no DJGPP-target compile and no layout measurement**: the
arm flip is rustc on this machine with two `--cfg`s, and its 53 errors are **all** in the mirror
graph or the 22 names the mirrors lack — the unit's own text contributes none of its own (Evidence
②).  The two `E0308`s that repair left behind are named in Deviations 25 rather than hidden.  The
file's `#if` verdicts are `tools/check-cond-eval.py`'s (and it is `[VERIFIED]` for this file), not
clang's.  C 2423-4350 is **not ported here**; the vacuously-passing `extract-abi.py --check`, the
22 missing names of Evidence ②, the 11 owed items and the 26.2% of the file left are that
boundary, not defects.  The `msdos` arm does not compile today, and that is the mirror graph's
state, not these slices'.
