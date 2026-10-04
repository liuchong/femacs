# emacs/src/indent.c (2592 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 7
- [x] current-column  (Fcurrent_column, 0/0, doc ca8f97fc098e, line 298)
- [x] indent-to  (Findent_to, 1/2, doc 9d29665ed86c, line 947)
- [x] current-indentation  (Fcurrent_indentation, 0/0, doc dc40025351d5, line 997)
- [x] move-to-column  (Fmove_to_column, 1/2, doc 5de5c991fcf1, line 1112)
- [x] compute-motion  (Fcompute_motion, 7/7, doc d2234e9da88e, line 1851)
- [x] line-number-display-width  (Fline_number_display_width, 0/1, doc 259ad48b0a17, line 2150)
- [x] vertical-motion  (Fvertical_motion, 1/3, doc 60fd0183eb91, line 2207)

## Functions defined — 19
- [x] buffer_display_table  (line 59)
- [x] character_width  (line 76, static)
- [x] disptab_matches_widthtab  (line 110)
- [x] recompute_width_table  (line 127)
- [x] width_run_cache_on_off  (line 145, static)
- [x] skip_invisible  (line 217)
- [x] invalidate_current_column  (line 323)
- [x] current_column  (line 329)
- [x] check_display_width  (line 481, static)
- [x] scan_for_column  (line 607, static)
- [x] current_column_1  (line 857, static)
- [x] position_indentation  (line 1013, static)
- [x] indented_beyond_p  (line 1101)
- [x] compute_motion  (line 1258)
- [x] vmotion  (line 1968)
- [x] line_number_display_width  (line 2097)
- [x] window_column_x  (line 2180, static)
- [x] restore_window_buffer  (line 2196, static)
- [x] syms_of_indent  (line 2577)

## Macros defined — 2
- [x] CR  (line 35, body 37444e63907d)
- [x] MULTIBYTE_BYTES_WIDTH  (line 283, body b26ca9ebd229)

## Types and their fields — 0

## File-scope objects — 5
- [x] static ptrdiff_t last_known_column  (line 43, static)
- [x] ptrdiff_t last_known_column_point  (line 47)
- [x] static modiff_count last_known_column_modified  (line 51, static)
- [x] static struct position val_compute_motion  (line 1192, static)
- [x] static struct position val_vmotion  (line 1965, static)

## Decisions

- **What the file carries.** C's 7 `DEFUN`s, its 19 functions (11 non-`static`:
  `buffer_display_table`, `disptab_matches_widthtab`, `recompute_width_table`,
  `skip_invisible`, `invalidate_current_column`, `current_column`, `indented_beyond_p`,
  `compute_motion`, `vmotion`, `line_number_display_width`, `syms_of_indent`), its two
  macros (`CR`, `MULTIBYTE_BYTES_WIDTH`), no type of its own, the five file-scope objects
  and the one `DEFVAR_BOOL` storage this unit owns (§3.29: `indent_tabs_mode`, which C
  keeps as `globals.f_indent_tabs_mode`; the extractor's "File-scope objects" list cannot
  see it, which is why `tools/check-globals.py` exists).  The 11 non-`static` definitions
  carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the eight `static` helpers stay plain
  `unsafe fn`, except `restore_window_buffer`, whose address C takes at both
  `record_unwind_protect` call sites, so it is `unsafe extern "C" fn` (PORTING.md 1.4).
- **Dead arms recorded, not transcribed (P-08).** `src/indent.c` is in
  `docs/baseline/config/cond-eval-verified.txt` and `python3 tools/check-cond-eval.py
  src/indent.c` reports `5 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the
  frozen configuration's branch selection is clang's.  The file's arms are:
  * the literal `#if 0` at C 252-260 (`skip_invisible`'s multibyte-form boundary walk,
    inside a live function — the port writes the surrounding statements and records the
    block);
  * the literal `#if 0 /* Not used.  */` at C 867-944, the whole `string_display_width`
    function — dead text, so it has **no Rust counterpart at all**;
  * the three `#ifdef HAVE_WINDOW_SYSTEM` arms (C 1334-1336 and 1341-1344 in
    `compute_motion`, C 1947-1949 in `Fcompute_motion`); `HAVE_WINDOW_SYSTEM` is `undef`
    in the frozen `config.h` (macros.tsv 1647), so C's live text at 1334-1336 is the
    unconditional `width -= 1;` — the port writes exactly that, and the two arms are
    recorded here.
  None of them is a *constant condition* that needs `if HAVE_X { … }`, and none is
  structural, so no `#[cfg (feature = …)]` text is carried; the row in
  `docs/ports/line-drift-tolerance.tsv` therefore declares the `live` basis.
- **`indent.h`'s mirror landed with this unit** (`femacs/src/include/indent.rs`, the
  charset.h/intervals.h precedent): `struct position`, its five prototypes and the one
  `extern` object.  This unit defines all five prototypes, so they moved into an
  `indent_decls` module + glob import (PORTING.md 0.1-10) and `last_known_column_point`
  into `indent_objs` (P-21).  `indent.h` includes only `lisp.h`, so the two prototypes
  that name `struct window` (C 34-40) and `recompute_width_table`'s `struct buffer *`
  (C 54-55) spell them the ABI-equal `*mut c_void` with a `C:` line (PORTING.md 0.1-2/P-72
  rule 3); the definitions in the unit spell the real types and shadow the glob import.
  `struct position` is a plain record and derives `Copy` (0.1-9): `vmotion`'s and
  `Fvertical_motion`'s `pos = *…` copy it.  Its size and offsets were measured with clang
  against the frozen target (48/8; `bufpos` 0, `bytepos` 8, `hpos` 16, `vpos` 24,
  `prevhpos` 32, `contin` 40) and pinned by the mirror's `const _` assertions, because
  `tools/gen-layout-fields.py`'s `STRUCTS` list does not name `position` yet (adding it is
  a follow-up for the layout ratchet, not something this unit can do behind the tool's
  back).
- **Two integrator edits to other people's files** (both reported, both the
  "the change lands in the same commit as the definition" rule, PORTING.md P-70/§3.53-3.56
  class):
  1. `femacs/src/include/disptab.rs`: `buffer_display_table` (disptab.h 70) is *this unit's*
     definition, and a same-named declaration at the mirror's root is one `E0428` in the
     crate that defines it.  The one-line declaration moved into a new `pub mod
     disptab_decls` + glob import; `window_display_table` (window.c's) stays at the root.
  2. `femacs/src/include/intervals.rs`: `TEXT_PROP_MEANS_INVISIBLE`'s **first arm matched
     the literal token `(prop)`**, so only a call site that spelled the token `prop` could
     use it — `indent.c`'s four calls (`TEXT_PROP_MEANS_INVISIBLE (prop)`,
     `… (propval)`) failed with `cannot find value `prop``.  The arm was an exact duplicate
     of the `$prop:expr` arm below it, so it is deleted; the mirror's own unit
     (`intervals.rs`, which passes the literal `prop`) is unaffected.
- **Reported mirror gaps** (declared at the use site in this unit, each with C's prototype
  in a `/// C:` line; nothing else was touched):
  * `lisp.h`: `find_newline` (5035), `find_newline_no_quit` (5040), `find_before_next_newline`
    (5042), `save_restriction_save` (4924), `save_restriction_restore` (4925),
    `labeled_restrictions_remove_in_current_buffer` (4931),
    `record_unwind_protect_void` (4825), `plist_get` (4280).
  * `window.h`: `decode_live_window` (1210), `window_internal_height` (1216),
    `window_body_width` (1223 — the mirror carries `enum window_body_unit` but not the two
    functions), `wset_buffer` (1231), `selected_window` (1100), and the `WINDOW_FRAME`
    macro (543), written as `macro_rules! WINDOW_FRAME`.
  * `region-cache.h`: its mirror is `include/region-cache.rs` now (status 3.74, C 33 in the
    include list above), so its four prototypes (`new_region_cache` 68, `free_region_cache` 71,
    `know_region_cache` 76, `region_cache_forward` 105) come from there with `struct
    region_cache *` spelled `*mut c_void` (0.1-2/P-72 rule 3) and `width_run_cache_on_off`'s
    return type is the same ABI-equal pointer; the declarations that stood here went away with
    the mirror.
  * `<limits.h>`: `SHRT_WIDTH` (limits.in.h 115) is not in the limits mirror (P-26); it is
    derived there the way that mirror derives its own widths.
  * `dispextern.h`: the mirror stops at its batch-II prefix, and `indent.c` is the first
    unit that drives the display iterator.  Declared here, with clang's measurements:
    `struct it` (2430-2934; **sizeof 4968**, align 8, and every reached offset asserted),
    `struct bidi_it` (2159-2222; sizeof 2328, `scan_dir` 184),
    `struct iterator_stack_entry` (2594-2665; sizeof 304, `method` 268),
    `struct composition_it` (2386-2426; sizeof 88, every field), the `it_method`,
    `line_wrap_method` and `move_operation_enum` enumerations, the position macros
    (`BYTEPOS`/`CHARPOS`/`SET_TEXT_POS`/`SET_TEXT_POS_FROM_MARKER`/`IT_CHARPOS`/
    `IT_BYTEPOS`/`IT_STRING_BYTEPOS`), `OVERLAY_STRING_CHUNK_SIZE` and `IT_STACK_SIZE`,
    and the ten entry points `indent.c` calls (`move_it_to`, `start_display`,
    `move_it_by_lines`, `move_it_in_display_line`,
    `reseat_at_previous_visible_line_start`, `bidi_shelve_cache`, `bidi_unshelve_cache`,
    `unwind_display_working_on_window`, `display_working_on_window_p`).  `struct it` is a
    **partial** record in the composite.rs `struct it`/`struct font` shape: the members
    `indent.c` reads or writes are named at C's measured offsets (`window` 0, `w` 8, `f` 16,
    `method` 24, `bits` 112, `current` 336, `string_overlays` 528, `string` 656,
    `from_overlay` 664, `stack` 672, `sp` 2192, `line_wrap` 2220, `c` 2228, `cmp_it` 2240,
    `image_id` 2336, `first_visible_x` 2436, `last_visible_x` 2440, `area` 2480,
    `current_x` 2524, `continuation_lines_width` 2532, `current_y` 2552, `vpos` 2560,
    `hpos` 2564, `lnum_width` 2584, `lnum_pixel_width` 2588, `bidi_it` 2616) and every gap
    is the measured pad, so the declaration's `sizeof` is C's — which it must be, because
    xdisp.c's functions write into the record through a pointer this unit passes.
- **The one bitfield this unit touches.** `struct it`'s first `bool_bf` run (C 2459-2471) is
  one byte at 112; the only member reached is `string_from_display_prop_p` (bit 3, measured:
  the byte reads `08` with only that bit set).  It is read through the member-name macro
  `it_bits! (string_from_display_prop_p, it)` over `IT_STRING_FROM_DISPLAY_PROP_P`
  (PORTING.md 1.3/P-80), and `docs/ports/token-aliases.tsv` carries the family with its
  occurrence check so deleting one of `Fvertical_motion`'s three reads still fails.
- **Uninitialised locals (PORTING.md 1.5) and `UNINIT`:** `Fvertical_motion`'s
  `double start_col`/`int start_x` are the `UNINIT` pair (recorded in
  `docs/ports/macro-body-notes.tsv` together with the other seven rows this unit needs);
  `struct position pos` is `std::mem::zeroed ()` in `vmotion` and `Fvertical_motion`
  (`Fvertical_motion`'s C only assigns `hpos`/`vpos` in one arm), and `struct it`,
  `struct text_pos` and `struct composition_it` locals are zeroed before
  `start_display`/`composition_compute_stop_pos` fill them, exactly as C's `memset
  (&cmp_it, 0, sizeof cmp_it)` does (the port keeps `memset` itself — PORTING.md 3.3 —
  and C's `sizeof` is `std::mem::size_of::<composition_it> ()`).
- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 53-54
  (`current_column_1`, `position_indentation`) and C 469 (`restore_window_buffer`); the
  definitions stay where C has them.
- **`DEFVAR_BOOL` storage (§3.29):** `indent_tabs_mode` is defined here with
  `#[unsafe(no_mangle)] pub static mut … : bool = false;` because C's storage is
  `globals.f_indent_tabs_mode` and this file owns the variable; `Findent_to` reads it and
  `syms_of_indent` sets it.  `tools/check-globals.py` reports zero variables without a
  definition in their own unit after this change.
- **`CR` is written `13`.** C 35 is `#define CR 015`; `tools/extract-abi.py`'s constant
  rule normalises C's octal literal to its decimal value (`015` → `13`) but cannot read
  Rust's `0o15` (its `scan_const_value`/`norm_const` take `[0-9]+`, `0x…`, `0b…` and C's
  leading-zero octal, not `0o…`), so the declaration is the decimal value and the octal
  spelling is recorded here.  The two *body* uses (`c == 0o15` in `character_width`,
  `c == CR` in `compute_motion`) stay octal, which the token scanner reads by value.
- **`goto`, `switch` and the other spelling rows** are in
  `docs/ports/macro-body-notes.tsv` (8 rows): `current_column`/`scan_for_column`/
  `compute_motion`'s `goto label` is a labelled block's `break 'label` (PORTING.md 2.2, the
  intervals.rs `'check_done` shape), `position_indentation`'s `switch (*p++)` is a `match`
  on the byte it advances past (`switch`/`case`/`default`/`break`/`FALLTHROUGH` noted),
  `compute_motion`'s `struct region_cache *width_cache` is `*mut c_void`,
  `indented_beyond_p`'s `allow_quit` `0` and `syms_of_indent`'s `indent_tabs_mode = 1` are
  Rust `false`/`true`, and `DEFVAR_BOOL!`'s `doc:` label has no counterpart (it is a macro
  parameter).  `C's` `do { … } while (cond)` in `compute_motion`'s invisible-line skip is a
  `loop { …; if !(cond) { break; } }`.
- **Spellings that stay closest to C**: `*++ptr`/`*--ptr`/`p[-1]`/`*p++` are
  `.offset (±1)` with the dereference in the same statement; `XSETFASTINT`/`XSETINT`/
  `XSETBUFFER`/`XSETWINDOW` keep C's macro names; `max`/`min` are lisp.rs's functions;
  `params C assigns to` are shadowed `let mut x = x;` (the cmds.rs/marker.rs shape), which
  is why `Findent_to`, `position_indentation`, `indented_beyond_p`, `compute_motion`,
  `vmotion`, `restore_window_buffer`, `Fvertical_motion`, `Fcompute_motion` and
  `Fmove_to_column` carry such shadows; the pointer/tag comparisons cast the way the
  mirrors force (`XMARKER (…).buffer as *mut buffer`, `w as *mut c_void` at the composite
  and buffer mirrors' `struct window *` parameters, `-1 as c_char` where C passes a
  `signed char`); `(int)`/`(double)`/`(EMACS_INT)` conversions C performs implicitly are
  explicit `as` casts, and every one of them is value-preserving for the values C feeds
  them.
- **`line` and `column` counts:** C 2592 lines, live 2501; the port is 2969, which
  `docs/ports/line-drift-tolerance.tsv` declares with the `live` basis (18.7%) and the
  reason; the per-function token comparison is the real guard and it is `OK`.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the
same argument order, and the port adds no call the reference does not make: the mirror
expansions (`BVAR!`, `FETCH_BYTE!`, `DISP_CHAR_VECTOR`, `IT_*`, `MULTIBYTE_BYTES_WIDTH!`)
are C's own macros expanding at the same points, and the three `*mut c_void` spellings
(`composition_reseat_it`/`composition_compute_stop_pos`/`composition_update_it`,
`overlay_strings`, `window_display_table`'s callers) change a parameter's declared type, not
the call graph.

### Integrator addition (2026-10-04, window.h's geometry family)

- The use-site declarations this ledger recorded for the `WINDOW_FRAME` macro are the mirrors' now: the window.h
  geometry macros (and, where they were here, the dispextern.h `MATRIX_*_LINE_HEIGHT`/
  `CURRENT_*_LINE_HEIGHT` family) moved into `include/window.rs`/`include/dispextern.rs` with
  window.h's third slice and the local copies were deleted (PORTING.md P-26;
  `docs/ports/include-window.md`'s "Decisions (third slice, 2026-10-04: the geometry family)"
  has the full list and the reason the family keeps C's `macro_rules!` shape).  Call sites keep
  C's text; `estimate_mode_line_height` (dispextern.h 3582) stays a use-site declaration here.
