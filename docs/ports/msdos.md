# emacs/src/msdos.c (4,350 lines) — first slice: C 1-1295

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
> (**1767 lines**, 69,140 bytes, md5 `32ace155550390f10b2135538e1b6c79`), carrying **C 1-1295**
> with **both arms of every `#if`**.
>
> **This ledger replaces, not extends, what it said before.**  Until this batch it read "the
> frozen configuration does not compile the file ⇒ the port is the empty unit plus the note that
> names the gate" (the P-08 reading).  P-86 ① replaces that with *transcribe the text, both arms,
> no stub*; what the earlier note measured (not in the frozen source list, empty checklist, a
> 336-byte object) is unchanged and still true — it is the *conclusion* that is withdrawn, exactly
> as `docs/ports/w32proc.md`, `docs/ports/w32.md` and `docs/ports/androidvfs.md` record for their
> own files.  The reference is **not valid UTF-8** (it is `coding: cp850`); it is read with
> `errors="replace"` and the eight non-ASCII lines are all at C 1972-2011, far past this slice.
>
> **This slice is C 1-1295**, from the file's first line through `IT_update_end`'s closing brace
> (C 1288-1295): the notes and the include list, the `SYS_ENVIRON` pair, the allocator triple, the
> whole mouse-control half (`event_timestamp`, `mouse_on`/`mouse_off`, `mouse_setup_buttons`, the
> `msdos-set-mouse-buttons` subr, `mouse_get_xy`, `mouse_moveto`, `mouse_pressed`/`mouse_released`/
> `mouse_button_depressed`, `mouse_get_pos`, `mouse_check_moved`, `mouse_clear_clicks`,
> `mouse_init`) and the first half of the screen half (both `#ifndef HAVE_X_WINDOWS` blocks'
> opening items, `dosv_refresh_virtual_screen`, `dos_direct_output`, `bright_bg`,
> `maybe_enable_blinking`, `vga_installed`, `dos_set_window_size`, `mouse_off_maybe`,
> `msdos_set_cursor_shape`, `IT_set_cursor_type`, `IT_ring_bell`, `IT_set_face`,
> `IT_write_glyphs`, `mouse_preempted`, `popup_activated`, `IT_write_glyphs_with_face`,
> `tty_draw_row_with_mouse_face`, `IT_clear_end_of_line`, `IT_clear_screen`, `IT_clear_to_end`,
> `IT_cursor_to`, `IT_display_cursor`, `IT_cmgoto`, `IT_update_begin`, `IT_update_end`).
> **The next live definition is C 1297-1333's `IT_frame_up_to_date`** (C 1296 is its blank
> line), and the remainder (C 1296-4350) is the table's last row: the keyboard maps, the
> DOS/V screen buffer, the file-name layer (`unixtodos_filename` and friends), the subprocess
> layer and `syms_of_msdos`, none of it ported here.
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
| C 480 `#ifndef HAVE_X_WINDOWS` … C 1771 `#endif` | the same pair, on every item of C 482-1295 | the same |
| C 1171 `#if 0` … C 1211 `#endif` (inside `IT_cmgoto`) | dead in *every* configuration; C's text is kept as a comment (the `src/cm.rs` 86 / `src/character.rs` 118 `#if 0` shape) | never |

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
| 1296-4350 | the unported remainder (C 1297-4350; `IT_frame_up_to_date` onward) | not ported this batch |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos1/cargo cargo run -q -p xtask -- build-tu src/msdos.c`
→ `built src/msdos.rs -> ../build/femacs/msdos.o`, **exit 0**, and `build/femacs/msdos.o` is
**336 bytes** (sha256 `d1ea88b2fe3d9609…`) — an empty object.  That is the expected reading, not a
proof of the new text: every item sits behind a `msdos` gate, so under the frozen feature set
rustc compiles only `include/config.rs` and the `#![allow]` attribute.  (It is also the P-86 ⑧
reading: an `include!` outside the gate would break this compile at once.)

**② The `msdos` arm flip, as a per-file error census (P-86 ③⑴/⑦).**  There is no DJGPP target to
compile for, so this is rustc on *this* machine with the derived feature set:
`cd femacs && CARGO_TARGET_DIR=/tmp/msdos1/cargo cargo run -q -p xtask -- probe src/msdos.rs
/tmp/msdos_arm6.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"' --cfg
'feature="dispextern_face_records"'` → **exit 1, 21 errors, 20 warnings**, and **not one of the
errors is a behaviour bug in this unit's text**:

* **16 × `E0428` "defined multiple times", all with the mirror as the *primary* span**: eight are
  `include/msdos.rs` against this unit — `dos_set_window_size` (88), `have_mouse` (116),
  `mouse_init` (117), `mouse_on` (118), `mouse_off` (119), `mouse_moveto` (120),
  `the_only_display_info` (250), `the_only_tty_output` (251) — i.e. C's "prototype in `msdos.h`,
  definition in `msdos.c`" pair, which Rust spells only if the declaration sits in a
  glob-imported module (PORTING.md 0.1-10's `syssignal_decls` shape; Owed 3).  The other eight are
  `include/msdos.rs`'s *overrides* against the port's system mirrors —
  `readlinkat`/`fstatat`/`faccessat`/`fchmodat`/`futimens`/`utimensat` (`lib/include/unistd.in.rs`
  39-41, `lib/include/sys_stat.in.rs` 181-196), `EINPROGRESS` (`lib/include/errno.rs` 92) and
  `O_CLOEXEC` (`lib/include/fcntl.in.rs` 118) — the class `docs/ports/w32proc.md` Owed 5(a)
  records for its own graph (Owed 2).
* **5 × `E0425` "cannot find", and each is a *mirror* gap, named**: `clock_t` (C 422; no mirror in
  this tree declares it at all — `lib/include/time.in.rs` is the `<time.h>` mirror and does not
  carry it), `make_formatted_string` (C 555; `lisp.h`'s mirror omits it), `cursor_to` (C 1016;
  `dispextern.h`'s mirror omits it), `clear_mouse_face` (C 1276; `dispextern.h`'s mirror omits it)
  and `face.tty_reverse_p` (C 835; the `struct face` copy omits the flag) — Owed 4.

So the unit's own text contributes **0 errors** to that reading *beyond those 5 mirror gaps*, and
the 16 `E0428`s cannot be decided from this file: the mirror graph has never been compiled with
`msdos` on.  Progression, for the record: the first flip (no `dispextern_face_records`) read 66
errors, the second 38, the third 32, the fourth 24, and the final pair-with-`countof`-and-`use`
edits 21 — every drop came from a *unit-side* spelling fix (the bit-field accessors the tree keeps
per consumer unit, `used[]`'s `c_short`, the untyped pointers the P-72 ③ mirrors carry, `mut` on
the two C parameters the port mutates, `countof!`'s `usize`-vs-`int` comparison, and the two
`use` edges Owed 5/6 describe), not from a change of behaviour.

**③ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/msdos.c` reads
**`src/msdos.c: 27 conditions, 0 unknown, 0 disagreements [VERIFIED]`** — every one of the file's
27 conditions is decided by the frozen table *and* agrees with clang.  Nine of them are in this
slice: C 34 (`MSDOS`), C 62 (the djgpp pair), C 95 (`_USE_LFN`), C 99 (`_dos_ds`), C 109
(`SYSTEM_MALLOC`), C 111 (`GNU_MALLOC`), C 119 (`REL_ALLOC`), C 402 and C 480 (`HAVE_X_WINDOWS`);
the other eighteen are past C 1295.  Both arms of each are carried in the port and each row of
the `## Gate mapping` table names the reading it came from.  The per-condition answer for this
file is *"the false arm, for all 27"*, because C 34 is the outermost condition — the reading was
taken with the evaluator's own `dead_branch_map` (`-1` meaning "the frozen configuration compiles
the else/dead side"), not from memory.

**④ `extract-abi.py --check` (vacuous here, and said so).**
`python3 tools/extract-abi.py --check emacs/src/msdos.c femacs/src/msdos.rs` → `OK
emacs/src/msdos.c -> femacs/src/msdos.rs`, exit 0.  This is P-86 ⑩'s empty checklist (the
reference's own live-item lists are all zero, Evidence above), *not* a comparison of the 36 functions and the
one subr the port carries.

**⑤ The other gates.**
`python3 tools/check-declarations.py` → exit 0 (0 findings; 682 files read).
`python3 tools/check-mirror-dupes.py` → `19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)`.
`python3 tools/check-ledger-ticks.py` → `234 ledger(s) bound …, 0 failure(s)` (this ledger ticks
nothing: its checklist is empty, so it is not bound).
`python3 tools/check-port-lines.py` → `70 file(s) under ten percent …, 0 new, 1 stale`
(`src/haiku_io.c`, another lane's): `src/msdos.c` is no longer under the threshold — this port
carries 1767/4350 = **40.6%** of the reference's lines.  `tools/check-dead-file-coverage.py` binds
this ledger through the `## C line ranges` section (its reading is in ⑥ below).

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

## Owed (nothing here is claimed as done)

1. **`femacs/xtask/Cargo.toml`**: three feature names this unit uses are registered nowhere —
   `gnu_malloc` (C 111), `_use_lfn` (C 95) and `_dos_ds` (C 99).  (`msdos`, `no_system_malloc`,
   `rel_alloc`, `have_x_windows` and `djgpp_2_3` are precedents that are registered.)
2. **`include/msdos.rs` must move its prototypes into a glob-imported module.**  Eight `E0428`s
   come from that mirror's top-level `extern "C"` blocks against this unit's definitions
   (`dos_set_window_size`, `have_mouse`, `mouse_init`, `mouse_on`, `mouse_off`, `mouse_moveto`,
   `the_only_display_info`, `the_only_tty_output`); the shape is `include/syssignal.rs` 29-52
   (`pub mod syssignal_decls` + `use syssignal_decls::*;`) and the rule is PORTING.md 0.1-10.
   Until it lands, **every** `msdos` translation unit that includes `msdos.h` and defines one of
   those names reports `E0428`.  The other eight `E0428`s of the same reading are that mirror's
   *overrides* against `lib/include/unistd.in.rs` 39-41, `sys_stat.in.rs` 181-196, `errno.rs` 92
   and `fcntl.in.rs` 118 (the `docs/ports/w32proc.md` Owed 5(a) class): the MS-DOS side is a
   *deliberate re-declaration* of the same name, so the reconciliation is the mirror round's
   (feature-mask the MS-DOS copy, as `include/msdos.rs` already does for `EINPROGRESS`/`O_CLOEXEC`
   at 84-87/80 — the pattern exists but the two files' rows have not been made consistent).
3. **`docs/baseline/port-lines-pending.tsv`**: this file's row (`src/msdos.c 4350 24 435`) is now
   stale — the port carries 40.6%, above the ten-percent threshold; the integrator's
   `check-port-lines.py --write-pending` re-run drops it.  (That file is not in this lane's
   writable set.)
4. **`lib/include/time.in.rs` (or `time.rs`): `clock_t`.**  C 422's `static clock_t startup_time;`
   has no counterpart — no mirror in this tree declares `clock_t` at all, and C reaches it through
   C 38's `#include <time.h>`, whose mirror is `time.in.rs`.  Until it lands the `msdos` arm
   reports `E0425` for the name.
5. **`include/dispextern.rs`: `cursor_to` and `clear_mouse_face` are missing**, and the header's
   own `struct face` copy lacks `tty_reverse_p` (C 835).  All three are declared in
   `dispextern.h`; the mirror carries neither function and its `face` copy omits the flag, so the
   arm reports three `E0425`s (plus one `E0609`).  `cursor_to` is `struct redisplay_interface`'s
   member in the reference, which is likely why it is absent.
6. **`make_formatted_string` (`lisp.h`) is missing from `include/lisp.rs`** — C 555's only use.
7. **The mirror round's reconciliation of the `dispextern_unit_names`/`frame_face_accessors`
   edges.**  Both modules are `pub mod`s inside a mirror that never glob-imports them
   (`include/syssignal.rs` 52 is the shape they lack), so *this* unit adds its own `use` edges
   (Deviations 10).  Either the mirrors add the glob or the per-consumer edges become the
   convention; the decision is the mirror round's, and until it is made every consumer names them
   itself.
8. **The layout assertions for the DJGPP records this unit spells** (`REGS`, `RegsX`, `RegsH`,
   `__dpmi_regs`, `_go32_info_block`) and the values of `_CRT0_FLAG_*`: `xtask layout-asserts`
   reads a C compiler's readings, and no DJGPP toolchain exists here, so not one `core::assert!`
   is written for them and the two flag constants stay `= 0` placeholders (Deviations 4).
9. **The next lane's slice: C 1296-4350**, whose first live definition is C 1297-1333's
   `IT_frame_up_to_date`, with the second `#ifndef HAVE_X_WINDOWS` arm running to C 1771 and the
   file's own new conditions starting at C 1784 (`#ifdef HAVE_X_WINDOWS`), C 1907/2433/2465/2787
   (`#ifndef HAVE_X_WINDOWS`), C 3565/3671 (`#ifdef HAVE_X_WINDOWS`), C 3691/3716
   (`#ifndef HAVE_X_WINDOWS`), C 3895/4135 (`__DJGPP__ == 2 && __DJGPP_MINOR__ < 4`), C 4128
   (`__DJGPP__ > 2 || __DJGPP_MINOR__ > 3`), C 4158 (`!HAVE_SELECT`), C 4252 (`#ifdef chdir`),
   C 4281 (`#ifdef tzset`) and C 4326 (`#ifndef HAVE_X_WINDOWS`).  Its keyboard maps are the file's only non-ASCII text
   (C 1972-2011).

## Not claimed

Every line of this slice sits behind gates the frozen configuration leaves false: ① is an empty
object, and the readings of ②-⑥ are compile-level or textual.  **Nothing here was executed** — no
`int86`/`intdos`/`__dpmi_int`, no BIOS call, no far-pointer store, no mouse, no screen write, no
`IT_*` entry point was ever called.  There is **no DJGPP-target compile and no layout
measurement**: the arm flip is rustc on this machine with two `--cfg`s, and its 21 errors are
**all** in the mirror graph or five names the mirrors lack — the unit's own text contributes none
of its own.  The file's `#if` verdicts are `tools/check-cond-eval.py`'s (and it is `[VERIFIED]`
for this file), not clang's.  C 1296-4350 is **not ported here**; the vacuously-passing
`extract-abi.py --check`, the 5 missing names of Evidence ②, the 8 owed feature/row items and the
59.4% of the file left are that boundary, not defects.  The `msdos` arm does not compile today,
and that is the mirror graph's state, not this slice's.
