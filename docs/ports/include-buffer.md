# emacs/src/buffer.h (1824 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 112
- [x] SET_PT  (line 170)
- [x] TEMP_SET_PT  (line 175)
- [x] SET_PT_BOTH  (line 180)
- [x] TEMP_SET_PT_BOTH  (line 185)
- [x] BUF_TEMP_SET_PT  (line 190)
- [x] BUFFERP  (line 757)
- [x] CHECK_BUFFER  (line 763)
- [x] XBUFFER  (line 769)
- [x] bset_bidi_paragraph_direction  (line 779)
- [x] bset_cache_long_scans  (line 784)
- [x] bset_case_canon_table  (line 789)
- [x] bset_case_eqv_table  (line 794)
- [x] bset_directory  (line 799)
- [x] bset_display_count  (line 804)
- [x] bset_left_margin_cols  (line 809)
- [x] bset_right_margin_cols  (line 814)
- [x] bset_display_time  (line 819)
- [x] bset_downcase_table  (line 824)
- [x] bset_enable_multibyte_characters  (line 829)
- [x] bset_filename  (line 834)
- [x] bset_keymap  (line 839)
- [x] bset_last_selected_window  (line 844)
- [x] bset_local_var_alist  (line 849)
- [x] bset_mark_active  (line 854)
- [x] bset_point_before_scroll  (line 859)
- [x] bset_read_only  (line 864)
- [x] bset_truncate_lines  (line 869)
- [x] bset_undo_list  (line 874)
- [x] bset_upcase_table  (line 879)
- [x] bset_width_table  (line 884)
- [x] bset_text_conversion_style  (line 890)
- [x] BUFFER_CEILING_OF  (line 901)
- [x] BUFFER_FLOOR_OF  (line 907)
- [x] BUF_BEGV  (line 917)
- [x] BUF_BEGV_BYTE  (line 925)
- [x] BUF_PT  (line 934)
- [x] BUF_PT_BYTE  (line 942)
- [x] BUF_ZV  (line 951)
- [x] BUF_ZV_BYTE  (line 959)
- [x] BUF_BEG  (line 970)
- [x] BUF_BEG_BYTE  (line 976)
- [x] BUF_GPT_ADDR  (line 983)
- [x] BUF_Z_ADDR  (line 990)
- [x] BUF_GAP_END_ADDR  (line 997)
- [x] BUF_COMPUTE_UNCHANGED  (line 1007)
- [x] SET_BUF_BEGV  (line 1034)
- [x] SET_BUF_ZV  (line 1041)
- [x] SET_BUF_BEGV_BOTH  (line 1048)
- [x] SET_BUF_ZV_BOTH  (line 1055)
- [x] SET_BUF_PT_BOTH  (line 1062)
- [x] BYTE_POS_ADDR  (line 1078)
- [x] CHAR_POS_ADDR  (line 1086)
- [x] CHAR_TO_BYTE  (line 1096)
- [x] BYTE_TO_CHAR  (line 1104)
- [x] PTR_BYTE_POS  (line 1112)
- [x] BUFFER_PVEC_INIT  (line 1135)
- [x] BUFFER_LIVE_P  (line 1143)
- [x] BUFFER_HIDDEN_P  (line 1152)
- [x] BUFFER_CHECK_INDIRECTION  (line 1160)
- [x] BUF_TS_LINECOL_BEGV  (line 1177)
- [x] BUF_TS_LINECOL_POINT  (line 1182)
- [x] BUF_TS_LINECOL_ZV  (line 1188)
- [x] SET_BUF_TS_LINECOL_BEGV  (line 1194)
- [x] SET_BUF_TS_LINECOL_POINT  (line 1200)
- [x] SET_BUF_TS_LINECOL_ZV  (line 1206)
- [x] decode_buffer  (line 1275)
- [x] set_buffer_internal  (line 1290)
- [x] record_unwind_current_buffer  (line 1300)
- [x] buffer_intervals  (line 1335)
- [x] set_buffer_intervals  (line 1344)
- [x] buffer_has_overlays  (line 1353)
- [x] FETCH_MULTIBYTE_CHAR  (line 1367)
- [x] BUF_FETCH_MULTIBYTE_CHAR  (line 1378)
- [x] FETCH_CHAR_AS_MULTIBYTE  (line 1391)
- [x] FETCH_CHAR  (line 1402)
- [x] BUF_BYTE_ADDRESS  (line 1413)
- [x] BUF_CHAR_ADDRESS  (line 1423)
- [x] BUF_PTR_BYTE_POS  (line 1433)
- [x] BUF_FETCH_BYTE  (line 1443)
- [x] BUF_FETCH_CHAR_AS_MULTIBYTE  (line 1453)
- [x] buffer_window_count  (line 1465)
- [x] overlay_start  (line 1476)
- [x] overlay_end  (line 1484)
- [x] OVERLAY_START  (line 1495)
- [x] OVERLAY_END  (line 1503)
- [x] OVERLAY_PLIST  (line 1511)
- [x] OVERLAY_BUFFER  (line 1519)
- [x] OVERLAY_REAR_ADVANCE_P  (line 1527)
- [x] OVERLAY_FRONT_ADVANCE_P  (line 1535)
- [x] PER_BUFFER_VALUE_P  (line 1575)
- [x] SET_PER_BUFFER_VALUE_P  (line 1585)
- [x] PER_BUFFER_IDX  (line 1612)
- [x] per_buffer_default  (line 1621)
- [x] set_per_buffer_default  (line 1627)
- [x] per_buffer_value  (line 1636)
- [x] set_per_buffer_value  (line 1642)
- [x] downcase  (line 1649)
- [x] upcase  (line 1658)
- [x] uppercasep  (line 1667)
- [x] lowercasep  (line 1674)
- [x] sanitize_tab_width  (line 1682)
- [x] SANE_TAB_WIDTH  (line 1689)
- [x] sanitize_char_width  (line 1697)
- [x] CHARACTER_WIDTH  (line 1708)
- [x] fetch_char_advance  (line 1723)
- [x] fetch_char_advance_no_check  (line 1749)
- [x] buf_next_char_len  (line 1769)
- [x] next_char_len  (line 1776)
- [x] buf_prev_char_len  (line 1785)
- [x] prev_char_len  (line 1794)
- [x] inc_both  (line 1802)
- [x] dec_both  (line 1812)

## Macros defined — 53
- [x] BEGV  (line 41, body c8dfe9c7da08)
- [x] BEGV_ADDR  (line 75, body eb5b453c5777)
- [x] BEGV_BYTE  (line 42, body 1a5b72c1341c)
- [x] BEG_ADDR  (line 72, body 53f96c84615c)
- [x] BEG_UNCHANGED  (line 156, body 855f47f3017f)
- [x] BUF_AUTOSAVE_MODIFF  (line 136, body 121f27c20c05)
- [x] BUF_BEG_ADDR  (line 117, body 9d16478b5cd5)
- [x] BUF_BEG_UNCHANGED  (line 149, body c8442338d88a)
- [x] BUF_BYTES_MAX  (line 199, body 03da28938830)
- [x] BUF_CHARS_MODIFF  (line 126, body 8a5ee5f3800d)
- [x] BUF_COMPACT  (line 139, body c1849c8c5c5a)
- [x] BUF_END_UNCHANGED  (line 150, body 41681c343561)
- [x] BUF_GAP_SIZE  (line 120, body 81659fb12fc0)
- [x] BUF_GPT  (line 109, body 1424d49a72a6)
- [x] BUF_GPT_BYTE  (line 110, body fa14f702252e)
- [x] BUF_MARKERS  (line 142, body 363d46a22f18)
- [x] BUF_MODIFF  (line 123, body b711c6f1b293)
- [x] BUF_OVERLAY_MODIFF  (line 132, body 11931c49e4ac)
- [x] BUF_OVERLAY_UNCHANGED_MODIFIED  (line 147, body bf79d1401038)
- [x] BUF_SAVE_MODIFF  (line 129, body 081acbf615dc)
- [x] BUF_UNCHANGED_MODIFIED  (line 144, body b95d7ce5b6bc)
- [x] BUF_Z  (line 113, body 567174119a24)
- [x] BUF_Z_BYTE  (line 114, body 0480de55ccde)
- [x] BVAR  (line 308, body cb903ab34bf2)
- [x] CHARS_MODIFF  (line 99, body 5559dc5fea88)
- [x] CHECK_FIXNUM_COERCE_MARKER  (line 1251, body ef5adbbb05af)
- [x] EMACS_BUFFER_H  (line 21, body da39a3ee5e6b)
- [x] END_UNCHANGED  (line 157, body f51497530ea3)
- [x] FETCH_BYTE  (line 219, body 64458896d8ca)
- [x] FOR_EACH_LIVE_BUFFER  (line 1329, body a586c01aa2d9) — deferred (P-22)
- [x] FOR_EACH_PER_BUFFER_OBJECT_AT  (line 1555, body 06ae1c125bed) — deferred (P-22)
- [x] GAP_END_ADDR  (line 84, body bc3191687a57)
- [x] GAP_SIZE  (line 93, body 1add0c898666)
- [x] GET_OVERLAYS_AT  (line 1311, body ff6daacd77db)
- [x] GPT  (line 50, body 8a1a3abbf391)
- [x] GPT_ADDR  (line 81, body 7cb80f245628)
- [x] GPT_BYTE  (line 51, body 0e22261b2251)
- [x] MODIFF  (line 96, body b26d13a1e5cf)
- [x] OVERLAY_MODIFF  (line 102, body c7af12a49405)
- [x] OVERLAY_UNCHANGED_MODIFIED  (line 154, body f7514b242610)
- [x] PER_BUFFER_VAR_IDX  (line 1566, body c3b6ab579f67)
- [x] PER_BUFFER_VAR_OFFSET  (line 1548, body 3af9f4472250)
- [x] PT  (line 46, body fa8ee60c12f2)
- [x] PT_ADDR  (line 78, body 25571d393b56)
- [x] PT_BYTE  (line 47, body 59b494a289bb)
- [x] SAVE_MODIFF  (line 105, body a964ef8bd0d0)
- [x] UNCHANGED_MODIFIED  (line 152, body c47ea34a043c)
- [x] Z  (line 58, body 1d98ebdf63a8)
- [x] ZV  (line 54, body d9bae3fef90c)
- [x] ZV_ADDR  (line 87, body c3830b53fe9f)
- [x] ZV_BYTE  (line 55, body a53a8fcf8ed7)
- [x] Z_ADDR  (line 90, body 1dc469aff198)
- [x] Z_BYTE  (line 59, body e56907e9cafd)

## Types and their fields — 4
- [x] buffer: union vectorlike_header header, Lisp_Object name_, Lisp_Object last_name_, Lisp_Object filename_, Lisp_Object directory_, Lisp_Object backed_up_, Lisp_Object save_length_, Lisp_Object auto_save_file_name_, Lisp_Object read_only_, Lisp_Object mark_, Lisp_Object local_var_alist_, Lisp_Object major_mode_, Lisp_Object local_minor_modes_, Lisp_Object mode_name_, Lisp_Object mode_line_format_, Lisp_Object header_line_format_, Lisp_Object tab_line_format_, Lisp_Object keymap_, Lisp_Object abbrev_table_, Lisp_Object syntax_table_, Lisp_Object category_table_, Lisp_Object tab_width_, Lisp_Object fill_column_, Lisp_Object left_margin_, Lisp_Object auto_fill_function_, Lisp_Object downcase_table_, Lisp_Object upcase_table_, Lisp_Object case_canon_table_, Lisp_Object case_eqv_table_, Lisp_Object truncate_lines_, Lisp_Object word_wrap_, Lisp_Object ctl_arrow_, Lisp_Object bidi_display_reordering_, Lisp_Object bidi_paragraph_direction_, Lisp_Object bidi_paragraph_separate_re_, Lisp_Object bidi_paragraph_start_re_, Lisp_Object selective_display_, Lisp_Object selective_display_ellipses_, Lisp_Object overwrite_mode_, Lisp_Object abbrev_mode_, Lisp_Object display_table_, Lisp_Object mark_active_, Lisp_Object enable_multibyte_characters_, Lisp_Object buffer_file_coding_system_, Lisp_Object file_format_, Lisp_Object auto_save_file_format_, Lisp_Object cache_long_scans_, Lisp_Object width_table_, Lisp_Object pt_marker_, Lisp_Object begv_marker_, Lisp_Object zv_marker_, Lisp_Object point_before_scroll_, Lisp_Object file_truename_, Lisp_Object invisibility_spec_, Lisp_Object last_selected_window_, Lisp_Object display_count_, Lisp_Object left_margin_cols_, Lisp_Object right_margin_cols_, Lisp_Object left_fringe_width_, Lisp_Object right_fringe_width_, Lisp_Object fringes_outside_margins_, Lisp_Object scroll_bar_width_, Lisp_Object scroll_bar_height_, Lisp_Object vertical_scroll_bar_type_, Lisp_Object horizontal_scroll_bar_type_, Lisp_Object indicate_empty_lines_, Lisp_Object indicate_buffer_boundaries_, Lisp_Object fringe_indicator_alist_, Lisp_Object fringe_cursor_alist_, Lisp_Object display_time_, Lisp_Object scroll_up_aggressively_, Lisp_Object scroll_down_aggressively_, Lisp_Object cursor_type_, Lisp_Object extra_line_spacing_, Lisp_Object ts_parser_list_, Lisp_Object text_conversion_style_, Lisp_Object cursor_in_non_selected_windows_, struct buffer_text own_text, struct buffer_text* text, ptrdiff_t pt, ptrdiff_t pt_byte, ptrdiff_t begv, ptrdiff_t begv_byte, ptrdiff_t zv, ptrdiff_t zv_byte, struct buffer* base_buffer, int indirections, int window_count, char local_flags, struct timespec modtime, off_t modtime_size, modiff_count auto_save_modified, modiff_count display_error_modiff, time_t auto_save_failure_time, ptrdiff_t last_window_start, struct region_cache* newline_cache, struct region_cache* width_run_cache, struct region_cache* bidi_paragraph_cache, bool_bf prevent_redisplay_optimizations_p : 1, bool_bf clip_changed : 1, bool_bf inhibit_buffer_hooks : 1, bool_bf long_line_optimizations_p : 1, struct itree_tree* overlays, struct ts_linecol ts_linecol_begv, struct ts_linecol ts_linecol_point, struct ts_linecol ts_linecol_zv, Lisp_Object undo_list_
- [x] buffer_text: unsigned char* beg, ptrdiff_t gpt, ptrdiff_t z, ptrdiff_t gpt_byte, ptrdiff_t z_byte, ptrdiff_t gap_size, modiff_count modiff, modiff_count chars_modiff, modiff_count save_modiff, modiff_count overlay_modiff, modiff_count compact, ptrdiff_t beg_unchanged, ptrdiff_t end_unchanged, modiff_count unchanged_modified, modiff_count overlay_unchanged_modified, INTERVAL intervals, struct Lisp_Marker* markers, bool_bf inhibit_shrinking : 1, bool_bf redisplay : 1
- [x] sortvec: Lisp_Object overlay, ptrdiff_t beg, ptrdiff_t end, EMACS_INT priority, EMACS_INT spriority
- [x] ts_linecol: ptrdiff_t bytepos, ptrdiff_t line, ptrdiff_t col

## File-scope objects — 6
- [x] extern struct buffer buffer_defaults  (line 1220)
- [x] extern struct buffer buffer_local_flags  (line 1236)
- [x] extern struct buffer buffer_local_symbols  (line 1242)
- [x] extern Lisp_Object interval_insert_behind_hooks  (line 1246)
- [x] extern Lisp_Object interval_insert_in_front_hooks  (line 1247)
- [x] extern Lisp_Object Vbuffer_alist  (line 1324)

## Decisions

`emacs/src/buffer.h` @ `fd7afced403`; port `femacs/src/include/buffer.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/buffer.h femacs/src/include/buffer.rs` -> `OK`
(one macro, `FOR_EACH_LIVE_BUFFER`, is recorded in `docs/ports/excluded-macros.txt`).

### Layout is proven, not assumed

`sizeof buffer == 984`, `sizeof buffer_text == 144`, `sizeof sortvec == 40`,
`sizeof ts_linecol == 24`, and the 99 `buffer.*` / 17 `buffer_text.*` `offsetof` keys
(101 and 19 keys with `sizeof`/`alignof`; 200 assertions in total, 7 keys still pending
a mirror) are asserted from the C compiler's numbers (`xtask layout-check`).  Two field
types needed care:

- `char local_flags[MAX_PER_BUFFER_VARS]` (a 50-byte array) is
  `[c_char; MAX_PER_BUFFER_VARS as usize]`; C's enum member is an `int`, so the length
  needs the cast, and the C spelling is kept in the name.
- `struct region_cache *newline_cache`/`width_run_cache`/`bidi_paragraph_cache`: C's
  `struct region_cache` is incomplete in this header, so the port keeps `*mut c_void`
  with the C spelling in a trailing comment, like `lisp.rs` does for `struct buffer *`
  (PORTING.md 0.1-2).
- The four `bool_bf` members of `struct buffer` and the two of `buffer_text` are packed
  into one `bits: u8` each (PORTING.md 1.3).  `bool_bf` is one byte on this target
  (`conf_post.h`), clang's record layout puts them at 960:0-960:3 and 136:0-136:1, and
  the mirror names them `BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P`, `BUFFER_CLIP_CHANGED`,
  `BUFFER_INHIBIT_BUFFER_HOOKS`, `BUFFER_LONG_LINE_OPTIMIZATIONS_P`,
  `BUFFER_TEXT_INHIBIT_SHRINKING`, `BUFFER_TEXT_REDISPLAY`.  `offsetof` may not name a
  bitfield, so the *bit assignment* is recorded in the constants' doc comments and
  checked by `spikes/write-probe.sh` (C writes the bitfields, Rust writes the masks, both
  print the raw bytes 15/3); the storage unit's width itself stays recorded, not proven
  (P-35).
- The four tree-sitter members (`ts_parser_list_`, `ts_linecol_begv/point/zv`) are behind
  `#[cfg(feature = "tree_sitter")]`, and the six accessors C gates on `HAVE_TREE_SITTER`
  with them.  A `const _: () = assert! (!cfg! (feature = "tree_sitter") ||
  HAVE_TREE_SITTER);` ties the feature to the frozen configuration, so enabling it without
  regenerating the configuration (which is what would add those members' layout keys)
  fails to compile instead of growing unasserted fields (P-08, P-34).
- `struct timespec`/`time_t`/`off_t` come from the WIP system mirrors
  `lib/include/time.in.rs` and `lib/include/sys_types.in.rs` (P-26).
- `modiff_count` and `INTERVAL` (`lisp.h` 4140/1423) live in `lisp.rs`; `INTERVAL` is an
  opaque pointer because `struct interval` is incomplete there.

### Shapes (PORTING.md P-37)

C's macro/function *kind* is kept: a macro in buffer.h becomes a `macro_rules!` of the
same name, an `INLINE` function becomes an `#[inline(always)]` function.

- The 26 object-like value macros (`BEGV`, `BEGV_BYTE`, `PT`, `PT_BYTE`, `GPT`,
  `GPT_BYTE`, `ZV`, `ZV_BYTE`, `Z`, `Z_BYTE`, `BEG_ADDR`, `BEGV_ADDR`, `PT_ADDR`,
  `GPT_ADDR`, `GAP_END_ADDR`, `ZV_ADDR`, `Z_ADDR`, `GAP_SIZE`, `MODIFF`, `CHARS_MODIFF`,
  `OVERLAY_MODIFF`, `SAVE_MODIFF`, `UNCHANGED_MODIFIED`, `OVERLAY_UNCHANGED_MODIFIED`,
  `BEG_UNCHANGED`, `END_UNCHANGED`) and the 20 argument-taking ones (`BUF_GPT`, …,
  `FETCH_BYTE`) are macros whose body is C's body.  This matters: the reference *writes*
  to `BEGV`/`GPT`/`GAP_SIZE`/`Z`/`ZV`/`MODIFF`/`CHARS_MODIFF`/`SAVE_MODIFF`/
  `BEG_UNCHANGED`/`END_UNCHANGED` in 159 statements, takes `&MODIFF` in 11, and assigns
  `FETCH_BYTE (pos)` once, and a `fn` is neither assignable nor addressable.  The cost is
  one spelling deviation: `NAME! ()` where C writes `NAME`.  `PT`/`PT_BYTE` keep C's
  `+ 0`, which is what makes them not l-values.
- `BUF_BEG`/`BUF_BEG_BYTE` stay functions (C spells them as `INLINE` functions returning
  the `BEG` constant), and `BUF_BYTES_MAX` stays a `const` (C's `#define` of a constant
  expression).
- `BVAR (buf, field)` / `PER_BUFFER_VAR_OFFSET (VAR)` paste a `_` onto the field name,
  which `macro_rules!` cannot do (`concat_idents!` is unstable).  The call site spells the
  member as Rust knows it — the C name plus the `_` C's own macro appends — and the macro
  only dereferences / takes `offset_of!`; `PER_BUFFER_VAR_IDX` composes the two unchanged.
- `FOR_EACH_PER_BUFFER_OBJECT_AT` is a numeric `for` loop whose body follows the macro at
  the C call site; the Rust body is the last argument (the shape `DEFUN!` uses) and the
  loop is a `while`.  Its iteration variable stays a macro argument.
- `GET_OVERLAYS_AT` is C's `do { … } while (false)`; it is a block macro and takes the
  enclosing function's `sa_avail` as its first argument, because C's `SAFE_NALLOCA` reads
  that name out of the expansion context and Rust macro hygiene does not.
- `CHECK_FIXNUM_COERCE_MARKER (x)` is an assignment *expression* in C; the port is a
  statement macro (no C caller uses the value).
- `OVERLAY_REAR_ADVANCE_P` / `OVERLAY_FRONT_ADVANCE_P` read itree's packed `bits` by mask
  (`ITREE_REAR_ADVANCE`/`ITREE_FRONT_ADVANCE`) where C names the bitfields.
- `current_buffer` is thread-local state in C (`thread.h` 147, reached through lisp.h).
  thread.rs is not written yet, so the mirror declares the storage
  (`unsafe extern "C" { static mut current_buffer: *mut buffer; }`) and the accessor
  replaces it when thread.rs lands — the same interim `lisp.rs` records for specpdl
  (P-19).  Both differential probes supply the symbol on the C and Rust sides.
- `buf_charpos_to_bytepos` / `buf_bytepos_to_charpos` are declared in `lisp.rs`, where
  `struct buffer` is incomplete, so their parameter is `*mut c_void` (0.1-2) and the
  `buffer.rs` call sites cast.
- `recenter_overlay_lists`, `fix_overlays_before` and `mmap_set_vars` are `extern` in the
  reference header but have no definition anywhere in the reference tree; the mirror
  carries the declarations with neutral parameter names.
- The four structs carry `#[repr(C, align(8))]`.  C has no alignment attribute on them
  and the natural alignment on this target is already 8 (every field is a pointer or
  `ptrdiff_t`); the explicit `align(8)` is recorded here because a 32-bit target would
  disagree with C's natural alignment of 4 and no assertion in the baseline would notice.
- C's include guard has no counterpart: `pub const EMACS_BUFFER_H: bool = true` exists so
  the mirror's name set matches the reference header's (0.1, P-24).
- Call-relationship differences (D7): none.  Parameter names in the `extern` blocks come
  from the definitions (`intervals.c`, `buffer.c`, `alloc.c`), because C's prototypes have
  none.

### Evidence

- `python3 tools/extract-abi.py --check emacs/src/buffer.h femacs/src/include/buffer.rs`
  -> `OK`.  What that proves: every C function/macro/type/field/global name is present,
  the struct field *sets* match, and no body is a stub.  What it does **not** prove (the
  first adversarial review measured this with a six-mutation matrix): function bodies,
  macro bodies and constant values are not compared, and `cinventory` extracts no
  constants from this header at all, because its one-line `enum { … };` forms are not
  parsed.  Those gaps are recorded as P-38 with the mutation matrix.
- `xtask layout-check`: 200 assertions over `sizeof`/`alignof`/`offsetof` (non-bitfield
  fields) plus the compile of every mirror.
- `spikes/write-probe.sh`: 4 lines identical — the 159 write sites' shapes (assignment to
  `GPT`/`ZV! …`, `+=`, `&raw mut MODIFF! ()`, `SAVE_MODIFF! () = MODIFF! ()`,
  `FETCH_BYTE! (1) = 0x41`), plus the six packed `bool_bf` bits through the masks
  (`bits 15 3`) and itree's three (`itree 7`).
- `spikes/accessor-probe.sh`: 11 lines identical — `BEGV`…`BUF_COMPACT`, six
  `BYTE_POS_ADDR`/`PTR_BYTE_POS`/`CHAR_POS_ADDR`/`BUF_PTR_BYTE_POS`/`FETCH_BYTE`/
  `BUF_FETCH_BYTE`/`CHAR_TO_BYTE` sweeps, the three `BUF_*_ADDR` offsets, and
  `BUF_COMPUTE_UNCHANGED` in both branches plus the min case.  Negative control: with the
  first branch's two stores swapped (a semantic error all three gates pass), the probe
  prints `unch eq 4 1` against C's `1 4`.
- A C-vs-Rust differential of the constants the review flagged as hand-derived: the C
  compiler and the mirror both print
  `2305843009213693950 75 47 984 2000` for `BUF_BYTES_MAX`, `BUFFER_LISP_SIZE`,
  `BUFFER_REST_SIZE`, `sizeof buffer` and `GAP_BYTES_DFL` (recipe:
  `clang -std=gnu23 -O1 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
  -I emacs/src -I emacs/lib probe.c` against a `rustc --edition 2024 -O probe.rs` that
  includes the mirrors with the `lispsym` arena `spikes/inline_probe.rs` uses).  It
  constrains five values, not the expressions that produce them.
- Not covered by any evidence yet: the accessors that need a real runtime
  (`BUF_BEGV`/`BUF_PT`/`BUF_ZV` through markers, `downcase`/`upcase` and the char-table
  readers, `buffer_intervals`, the overlay accessors, `GET_OVERLAYS_AT`'s allocation
  path, `CHARACTER_WIDTH`), and the `feature = "tree_sitter"` branch (P-30, P-38).

### Adversarial review of 2026-10-02 and its dispositions

Two reviewers attacked the frozen commit `f583f33` (`docs/reviews/2026-10-02-include-buffer-review.md`
records their findings and the evidence).  Everything they raised is resolved here or
recorded as an open item:

| Finding | Disposition |
| --- | --- |
| 45 object-like macros as `fn` cannot express C's 159 write sites, 11 `&MODIFF` and `FETCH_BYTE (pos) =` | macros of C's kind, place expressions; `spikes/write-probe.sh`; recorded in P-37 |
| `--check` does not compare function bodies, macro bodies or constants, and the enum constants of this header are not extracted at all (six-mutation matrix passed) | recorded as P-38 with the mutation matrix; the mirror's evidence section above states what each gate covers |
| `GET_OVERLAYS_AT` → `SAFE_NALLOCA` evaluated `&*p`, which aborts on the two real call sites' NULL/uninitialized pointers | `pointee_size ($buf)` (lisp.rs); recorded |
| the six `bool_bf` bits had neither names nor recorded bit numbers | six mask constants with clang's bit numbers; `spikes/write-probe.sh` |
| two C comments (the `buffer_text` bitfields) had been dropped | restored on those fields |
| the ledger's `99 / 19` mixed an `offsetof` count with a key count | corrected to 99 / 17 `offsetof` (101 / 19 keys) |
| `eassert!` dropped the condition text and hardcoded `lisp.rs` | `concat! (stringify! ($cond), "\0")` + `concat! (file! (), "\0")` |
| `FOR_EACH_PER_BUFFER_OBJECT_AT` was filed under P-22 although it needs no loop-macro decision | written, with the body as its last argument |
| the `cfg (feature = "tree_sitter")` was not tied to the frozen configuration | `const _` assertion |
| crate-root `extern` declarations in `lisp.rs` (74, of which ~62 are Emacs C functions) will collide with `E0428` when the defining unit lands | recorded as P-39 with the plan (move them into a `pub mod` + `use …::*`, the P-21/P-28 shadowing shape) |
| `#[repr(C, align(8))]`, the include-guard constant and the `buf_charpos_to_bytepos` parameter name | recorded / corrected above |
