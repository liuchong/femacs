# emacs/src/bidi.c (3727 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 53
- [x] bidi_get_type  (line 275, static)
- [x] bidi_check_type  (line 319, static)
- [x] bidi_get_category  (line 326, static)
- [x] bidi_isolate_fmt_char  (line 365, static)
- [x] bidi_mirror_char  (line 377)
- [x] bidi_paired_bracket_type  (line 418, static)
- [x] bidi_set_sos_type  (line 433, static)
- [x] bidi_push_embedding_level  (line 454, static)
- [x] bidi_pop_embedding_level  (line 483, static)
- [x] bidi_remember_char  (line 527, static)
- [x] bidi_copy_it  (line 543, static)
- [x] bidi_cache_reset_to  (line 610, static)
- [x] bidi_cache_reset  (line 623, static)
- [x] bidi_cache_shrink  (line 633, static)
- [x] bidi_cache_fetch_state  (line 645, static)
- [x] bidi_cache_search  (line 664, static)
- [x] bidi_cache_find_level_change  (line 729, static)
- [x] bidi_level_start  (line 773)
- [x] bidi_cache_ensure_space  (line 783, static)
- [x] bidi_cache_iterator_state  (line 819, static)
- [x] bidi_cache_find  (line 900, static)
- [x] bidi_peek_at_next_level  (line 926, static)
- [x] bidi_push_it  (line 949)
- [x] bidi_pop_it  (line 970)
- [x] bidi_shelve_cache  (line 998)
- [x] bidi_unshelve_cache  (line 1046)
- [x] bidi_initialize  (line 1117, static)
- [x] bidi_set_paragraph_end  (line 1149, static)
- [x] bidi_init_it  (line 1160)
- [x] bidi_line_init  (line 1201, static)
- [x] bidi_count_bytes  (line 1233, static)
- [x] bidi_char_at_pos  (line 1261, static)
- [x] bidi_fetch_char  (line 1289, static)
- [x] bidi_fetch_char_skip_isolates  (line 1433, static)
- [x] bidi_at_paragraph_end  (line 1482, static)
- [x] bidi_paragraph_cache_on_off  (line 1519, static)
- [x] bidi_find_paragraph_start  (line 1573, static)
- [x] find_first_strong_char  (line 1640, static)
- [x] bidi_paragraph_init  (line 1716)
- [x] bidi_explicit_dir_char  (line 1865, static)
- [x] bidi_resolve_explicit  (line 1888, static)
- [x] bidi_resolve_weak  (line 2237, static)
- [x] bidi_resolve_neutral_1  (line 2515, static)
- [x] bidi_find_bracket_pairs  (line 2617, static)
- [x] bidi_record_type_for_neutral  (line 2863, static)
- [x] bidi_resolve_brackets  (line 2885, static)
- [x] bidi_resolve_neutral  (line 3037, static)
- [x] bidi_type_of_next_char  (line 3236, static)
- [x] bidi_level_of_next_char  (line 3254, static)
- [x] bidi_find_other_level_edge  (line 3420, static)
- [x] bidi_move_to_visually_next  (line 3478)
- [x] bidi_find_first_overridden  (line 3648)
- [x] bidi_dump_cached_states  (line 3702)

## Macros defined — 12
- [x] BIDI_CACHE_CHUNK  (line 560, body 9f9af029585b)
- [x] BIDI_CACHE_MAX_ELTS_PER_SLOT  (line 580, body c2d4c5452f59)
- [x] BIDI_EOB  (line 254, body eb883130e83d)
- [x] CANONICAL_EQU  (line 2581, body 428298e06e7c)
- [x] FLAG_EMBEDDING_INSIDE  (line 2532, body 356a192b7913)
- [x] FLAG_OPPOSITE_INSIDE  (line 2533, body da4b9237bacc)
- [x] ISOLATE_STATUS  (line 448, body a5da382681fc)
- [x] MAX_PARAGRAPH_SEARCH  (line 1566, body 63c34a2a7797)
- [x] MAX_STRONG_CHAR_SEARCH  (line 1630, body 409e9519c662)
- [x] OVERRIDE  (line 449, body 557bdab60f35)
- [x] PUSH_BPA_STACK  (line 2594, body 0b322d7afcae)
- [x] STORE_BRACKET_CHARPOS  (line 2591, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 13
- [x] static bool bidi_initialized  (line 250, static)
- [x] static Lisp_Object bidi_type_table  (line 252, static)
- [x] static Lisp_Object bidi_mirror_table  (line 252, static)
- [x] static Lisp_Object bidi_brackets_table  (line 252, static)
- [x] static Lisp_Object paragraph_start_re  (line 265, static)
- [x] static Lisp_Object paragraph_separate_re  (line 265, static)
- [x] static ptrdiff_t bidi_cache_max_elts  (line 582, static)
- [x] static struct bidi_it bidi_cache  (line 583, static)
- [x] static ptrdiff_t bidi_cache_size  (line 584, static)
- [x] static ptrdiff_t bidi_cache_start_stack  (line 594, static)
- [x] static int bidi_cache_sp  (line 595, static)
- [x] static ptrdiff_t bidi_cache_total_alloc  (line 994, static)
- [x] static ptrdiff_t nsearch_for_strong  (line 1624, static)

## Decisions

- **What the file carries.** C's 53 functions (11 non-`static`: `bidi_mirror_char`,
  `bidi_level_start`, `bidi_push_it`, `bidi_pop_it`, `bidi_shelve_cache`,
  `bidi_unshelve_cache`, `bidi_init_it`, `bidi_paragraph_init`,
  `bidi_move_to_visually_next`, `bidi_find_first_overridden`,
  `bidi_dump_cached_states`; they carry `#[unsafe(no_mangle)]`, PORTING.md 0.2), the 42
  `static` helpers as plain `unsafe fn`, the twelve macros (eleven `#define`s plus
  `PUSH_BPA_STACK`'s helper `STORE_BRACKET_CHARPOS`), the `bidi_category_t` enumeration,
  the one type the file declares (`bpa_stack_entry`), its `elsz`,
  `bidi_shelve_header_size` and `MAX_BPA_STACK` enumerators, and the thirteen file-scope
  objects (all `static` in C, so all unexported `static mut`s here — `check-linkage.py`
  confirms `src/bidi` has no non-`static` object).  `bidi.c` has **no `DEFUN`** and owns
  no `DEFVAR_*` storage, so `check-globals.py` has nothing to check for it.
- **Dead arms (P-08) *and* the `ENABLE_CHECKING` arms (P-51).**  `src/bidi.c` is in
  `docs/baseline/config/cond-eval-verified.txt` and `python3 tools/check-cond-eval.py
  src/bidi.c` reports `4 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the frozen
  configuration's branch selection is clang's.  The file's **only** four conditions are
  `#ifdef ENABLE_CHECKING` (`ENABLE_CHECKING` is `undef`, macros.tsv 872): C 1654-1659
  (`find_first_strong_char`'s fetch-and-`eassert` before skipping an isolate),
  C 2540-2542 (`bpa_stack_entry.open_bracket_pos`), C 2587-2592
  (`STORE_BRACKET_CHARPOS`) and C 2691-2693 (the pairing-position `eassert` in
  `bidi_find_bracket_pairs`).  Unlike a plain dead arm these are **transcribed**, under
  `#[cfg (enable_checking)]`, because `xtask`'s `--cfg enable_checking` pass compiles
  exactly this text (§3.50's rule: "抄下更多 C 文本是更 1:1，不是更少"); the
  `bpa_stack_entry` assertion pair covers both configurations (sizeof 12/align 4 in the
  frozen one, sizeof 24/align 8 with `-DENABLE_CHECKING`, clang-measured), and the
  `--cfg enable_checking` compile was run and succeeds.  The line-count row therefore
  keeps the **`total`** basis.
- **`struct bidi_it` is declared here, complete** (`dispextern.h`'s mirror stops at its
  batch-II prefix, and `indent.rs` declares only the `scan_dir` prefix it reaches).
  clang against the frozen target: **sizeof 2328, align 8**, and every member's offset
  asserted (`bytepos` 0, `charpos` 8, `ch` 16, `nchars` 24, `ch_len` 32, `type` 40,
  `type_after_wn` 44, `orig_type` 48, `resolved_level` 52, `isolate_level` 53,
  `invalid_levels` 56, `invalid_isolates` 64, `prev` 72, `last_strong` 88,
  `next_for_neutral` 104, `prev_for_neutral` 120, `next_for_ws` 136,
  `bracket_pairing_pos` 152, `bracket_enclosed_type` 160, `next_en_pos` 168,
  `next_en_type` 176, `sos` 180, `scan_dir` 184, `disp_pos` 192, `disp_prop` 200,
  `stack_idx` 204, `level_stack` 208, `string` 2256, `w` 2296, `paragraph_dir` 2304,
  `separator_limit` 2312, `bits` 2320) plus the `level_stack` extent
  (`BIDI_MAXDEPTH + 2 + 1` = 128 entries × 16 = 2048, measured).  It derives `Clone,
  Copy` because C copies whole iterators with `=`.
  **`struct window *w` is `*mut c_void`** with a `/// C:` line (P-72 rule 3): as in C —
  where `window.h` is *not* in `bidi.c`'s include graph — `struct window` is incomplete in
  this unit, and the three entry points that take it declare the same pointer.
  `signed char resolved_level`/`isolate_level` are `c_char` (the composite.rs spelling:
  the mirrors carry no `c_schar`).
- **The three bitfield runs this unit reads and writes.**  `struct bidi_it`'s three
  `bool_bf : 1` members share the byte at 2320 (measured: `01` for `first_elt`, `02` for
  `new_paragraph`, `04` for `frame_window_p`) and go through `bidi_it_bits! (NAME, it)`,
  `struct bidi_stack`'s three 3-bit members are the mirror's `bits: u16` (measured `f5 01`
  for 5/6/7: `next_for_neutral_type` bits 0-2, `last_strong_type` 3-5,
  `prev_for_neutral_type` 6-8) and go through `bidi_stack_bits!`,
  `struct bidi_string_data`'s `from_disp_str`/`unibyte` are the mirror's `bits: u32`
  (measured `01`/`02`) and go through `bidi_string_bits!` — every read and write keeps C's
  member name (PORTING.md 1.3/P-80).  `bpa_stack_entry`'s `unsigned flags : 2` is
  `flags: c_uint` over the 4-byte unit at 8 (asserted), and `struct bidi_stack.flags`
  (sos/override/isolate_status) is a plain `u8` because C reads and writes it with shifts
  (`ISOLATE_STATUS`/`OVERRIDE`, `<< 3`), which the port keeps as macros with C's bodies.
- **Reported mirror gaps** (declared at the use site in this unit, each with C's prototype
  in a `/// C:` line; no mirror was touched):
  * `dispextern.h`: `struct bidi_it` (2159-2203), `IT_STACK_SIZE` (2384),
    `update_redisplay_ticks` (3721), `compute_display_string_pos` (3621),
    `compute_display_string_end` (3624) and the `SET_TEXT_POS` macro (291-292 — the
    position macros arrive with the display batch, the indent.rs shape).  **The integrator
    should move all six into `include/dispextern.rs`** (deleting `indent.rs`'s partial
    `bidi_it` in the same commit, P-70) when the display batch lands; nothing else in this
    unit's include graph reaches them.
  * `lisp.h`: `find_newline_no_quit` (5040) and `fast_looking_at` (5031).
  * `region-cache.h`: **no mirror at all**, so its four prototypes (`new_region_cache` 68,
    `free_region_cache` 71, `know_region_cache` 76, `region_cache_backward` 105) are
    declared here with `struct region_cache *` spelled `*mut c_void` (0.1-2/P-72 rule 3,
    the indent.rs/insdel.rs shape) — that is why `bidi_find_paragraph_start`'s token row
    lists `region_cache`.
  * `<intprops.h>`: `INT_PROMOTE (e)` = `(+ (e))` (intprops.h 55) as a `macro_rules!`
    (P-26, the ccl.rs `INT_DIVIDE_OVERFLOW` shape); C uses it six times to keep
    `-Wswitch-enum` quiet.
  * `<stdio.h>`: `fprintf` (bytecode.rs shape), `putc` (cm.rs shape) and `stderr` with its
    Darwin link name (`__stderrp`, region-cache.rs shape).
- **C constructs and how they are spelled** (each row is in
  `docs/ports/macro-body-notes.tsv`, 11 rows appended for this unit):
  * `switch`/`case`/`default` → `match` with C's arms in C's order (the `_` arm is C's
    `default:`), in `bidi_get_type`, `bidi_get_category`, `bidi_resolve_explicit`,
    `bidi_find_bracket_pairs`, `bidi_resolve_brackets`, `bidi_resolve_neutral`.
  * `goto normal_char` (C 1356) → the `normal_char` flag tested where C's label sits, so
    the body runs once and the label name survives (the regex.rs `regex_compile` shape).
  * the two `goto give_up`s (C 2674, 2773) → `break 'give_up` out of the labelled block
    that carries C's `give_up:` epilogue.
  * **`case FSI:`'s `FALLTHROUGH` into `case RLI:` and its `goto fsi_as_lri` into
    `case LRI:`** → the `'fsi_as_lri` dispatch loop: the FSI arm sets the arm name and
    `continue`s, so C's **two textual copies** of the shared RLI/LRI body each appear once
    and the family occurrence check sees C's counts (a single shared body would have been
    one `type`/`override`/… short, which is exactly what the first `--check` reported).
  * C's `INT_PROMOTE (x)` is kept as the macro name; `~1` is `!1`; C's `(int)`/`(char)`
    conversions are explicit `as` casts; `-1`/`1`/`0` for C's `bool` parameters are
    `false`/`true`.
  * C's chained assignments (`a = b = c;`) are one statement per assignment in C's
    right-to-left order (the composite.rs rule); `bidi_cache[i++] = *bidi_it`,
    `--bidi_cache_sp` and the `n++` inside `while (… n++ < MAX_PARAGRAPH_SEARCH …)` keep
    C's post-increment semantics (the latter as a block expression in the condition, so the
    short-circuit order is C's).
  * `%"pD"d` is `%td` (lisp.h 150's `pD` is `"t"`; the cmds.rs precedent) — the `pD` token
    itself is two characters wide, so the body comparison skips it as a short token.
  * `EXTERNALLY_VISIBLE` (C 3700, print.rs precedent) and C 370's forward declaration of
    `bidi_initialize` (0.1-12) are dropped.
- **`struct bidi_it`'s `w` and the three callees that take it** (`compute_display_string_pos`,
  `compute_display_string_end`, `update_redisplay_ticks`) all spell `struct window *` as
  `*mut c_void`; the call sites pass `bidi_it->w` unchanged.
- **Uninitialised locals (PORTING.md 1.5)** are the `std::mem::zeroed ()` declarations named
  here: `struct bidi_it saved_it`/`tem_it`/`sentinel` and `struct text_pos pos`
  (`bidi_fetch_char`), `saved_it` in `bidi_resolve_neutral`, plus `bidi_find_bracket_pairs`'
  `saved_it`, `tem_it`, `pairing_pos` and its `bpa_stack` array; `struct bidi_saved_info
  prev_for_neutral`/`next_for_neutral` in `bidi_resolve_brackets`; the `int len` of
  `bidi_fetch_char`'s three `string_char_and_length` calls, the `int ch` of
  `bidi_resolve_brackets`/`bidi_level_of_next_char`, the `int ch` of
  `bidi_fetch_char`'s `else` chain, `bidi_type_t type` in `bidi_paragraph_init`'s
  NEUTRAL_DIR arm, and the `ptrdiff_t idx` of `bidi_unshelve_cache` (filled through the
  `memcpy`).
- **`struct bpa_stack_entry` and `MAX_BPA_STACK`.**  The 977-entry `bpa_stack` local is
  `[bpa_stack_entry; MAX_BPA_STACK as usize]` (11.7 KiB of a 16 KiB `MAX_ALLOCA` frame, as
  in C); `elsz`, `bidi_shelve_header_size` and `MAX_BPA_STACK` keep C's arithmetic — the
  first two name C's own operands (`size_of::<bidi_it> ()` for `elsz`, `size_of_val (&OBJ)`
  for the six header fields, which needs the `unsafe` const block because they are
  `static mut`s), the third writes C's `max (1, …)` out as C's own ternary because Rust's
  `max` is not a `const fn` (the `BUF_BYTES_MAX` class).
  `PUSH_BPA_STACK` and `STORE_BRACKET_CHARPOS` capture `bidi_it`, `bpa_stack` and `bpa_sp`,
  so both are defined **inside** `bidi_find_bracket_pairs` after those names exist (C has
  them at file scope, C 2587-2606); a file-scope definition compiles in the frozen
  configuration only because the dead arm expands to nothing, and the `--cfg
  enable_checking` pass caught exactly that (E0425) — the ccl.rs P-57 rule.
- **`bidi_paragraph_cache_on_off`'s `struct region_cache *`** return is `*mut c_void` and
  its `bidi_paragraph_cache` field reads/writes spell C's `0` as `std::ptr::null_mut ()`.
- **Line counts.** C 3,727 lines by `wc -l` (3,728 by `tools/extract-abi.py`'s
  `split ("\n")` count, which is the one the drift rule uses; **3,716 live** — the twelve
  `#ifdef ENABLE_CHECKING` lines are the only ones the frozen configuration drops) → the
  port's **3,825** by `wc -l` (3,826 by the same count): **2.6 %** against the `total`
  basis and 2.9 % against live, inside the 5 % default, so **no row was added to
  `docs/ports/line-drift-tolerance.tsv`** (that table is unchanged by this unit).  The
  excess is the use-site declaration block (the complete `bidi_it` record with 33 `const _`
  assertions, `bpa_stack_entry` with five, the eleven use-site prototypes and the four
  macros), the Rust spellings of C's statement shapes (`let mut` shadows, `unsafe` blocks,
  explicit casts, the two dispatch loops) and the `macro_rules!` definitions.
- **Suspected defect in the reference (copied faithfully, not fixed).**  In
  `bidi_resolve_explicit`, C 1958 writes `bidi_it->first_elt = 0;` and C 1987 then tests
  `if (bidi_it->first_elt && bidi_it->prev.type != UNKNOWN_BT)`, which can never be true;
  `prev_type` therefore never picks up `bidi_it->prev.orig_type` on this path, although the
  comment above it says that is its purpose (the `#ifdef ENABLE_CHECKING` arm at C 1654-1659
  and the W1 resolution both rely on `prev`).  The port keeps C's order and the dead test.
- **The `--check`'s own findings were resolved by notes, not by edits to the port's
  semantics**: the first run reported the eleven rows above plus one family shortfall
  (`type` 30 vs 29) — the missing copy was C's second textual copy of the RLI/LRI arm body,
  which the `'fsi_as_lri` dispatch loop now carries.

### Call-relationship differences (D7)

Measured with `nm -u` on both objects (`clang -c` of `emacs/src/bidi.c` against the frozen
configuration vs `build/femacs/bidi.o`): **35 undefined references in C, 49 in the port
(38 after the Rust panic machinery), and 28 shared** — `buf_bytepos_to_charpos`,
`buf_charpos_to_bytepos`, `char_table_ref`, `compute_display_string_end`,
`compute_display_string_pos`, `emacs_abort`, `fast_looking_at`, `find_newline_no_quit`,
`fprintf`, `fputs`, `free_region_cache`, `intern_1`, `know_region_cache`, `make_string`,
`memcpy`, `new_region_cache`, `region_cache_backward`, `specbind`, `staticpro`, `strlen`,
`unbind_to`, `uniprop_table`, `update_redisplay_ticks`, `xfree`, `xmalloc`, `xpalloc`,
`xrealloc`, `stderrp`.  Every callee of C's source text is in the port's set (the inline
helpers `string_char_and_length`, `CHAR_TABLE_REF`, `SDATA`, `BYTES_BY_CHAR_HEAD`,
`BYTE_POS_ADDR`, `dec_both` … are expanded on both sides and reference nothing external).

The differences, all of them *mirror shape* or *codegen*, none a changed implementation
path:
* **C-only.**  `__chkstk_darwin`, `__stack_chk_fail`, `__stack_chk_guard` are clang's
  stack-probe/stack-protector helpers for C's 11.7 KiB frame, not source callees; `_globals`
  and `_lispsym` are how C reaches every `DEFVAR_*` (`globals.f_bidi_inhibit_bpa`,
  `globals.f_max_redisplay_ticks`) and every `Q*` symbol (`lispsym[…]`) — the port's mirrors
  declare each variable/`Q*` as its own name (status §3.29's storage model and globals.rs's
  generated `builtin_lisp_symbol (N)` accessors); `_current_thread` is C's `current_buffer`
  macro (`current_thread->m_current_buffer`), which the port spells with buffer.rs's
  `current_buffer` symbol; `_putc_unlocked` is C's `putc` macro expanding to the unlocked
  entry point, while the port declares `putc` itself (cm.rs shape).
* **Port-only.**  `_bidi_inhibit_bpa` and `_max_redisplay_ticks` (the same §3.29 storage
  difference, seen from the other side), `_current_buffer`, `_putc`, `___stderrp` (the
  Darwin link name), `_specpdl`/`_specpdl_ptr` (lisp.rs's `SPECPDL_INDEX`), and
  `_setjmp`/`_bzero`/`_cons_to_signed`/`_cons_to_unsigned` — the last four are emitted by
  `lisp.rs`'s `sys_setjmp`/`CONS_TO_INTEGER` inline family and are present in other units'
  objects too (measured in `build/femacs/indent.o`, `cmds.o` and `insdel.o`), i.e. crate
  artifacts rather than this unit's calls.  The remaining port-only names are Rust's core
  panic symbols, which every unit's object references.
