# emacs/src/composite.c (2322 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 6
- [x] clear-composition-cache  (Fclear_composition_cache, 0/0, doc 202b560674dd, line 692)
- [x] composition-get-gstring  (Fcomposition_get_gstring, 4/4, doc a65a3688b856, line 1904)
- [x] compose-region-internal  (Fcompose_region_internal, 2/4, doc 069c7569e032, line 2009)
- [x] compose-string-internal  (Fcompose_string_internal, 3/5, doc 6e3d6522f350, line 2029)
- [x] find-composition-internal  (Ffind_composition_internal, 4/4, doc f456ff242555, line 2048)
- [x] composition-sort-rules  (Fcomposition_sort_rules, 1/1, doc 8460fceeb687, line 2140)

## Functions defined — 26
- [x] get_composition_id  (line 166)
- [x] find_composition  (line 421)
- [x] run_composition_function  (line 461, static)
- [x] update_compositions  (line 493)
- [x] make_composition_value_copy  (line 599)
- [x] compose_text  (line 624)
- [x] composition_gstring_lookup_cache  (line 644)
- [x] composition_gstring_put_cache  (line 650)
- [x] composition_gstring_from_id  (line 673)
- [x] composition_gstring_cache_clear_font  (line 683)
- [x] composition_gstring_p  (line 707)
- [x] composition_gstring_width  (line 738)
- [x] composition_gstring_adjust_zero_width  (line 798)
- [x] fill_gstring_header  (line 844, static)
- [x] fill_gstring_body  (line 868, static)
- [x] autocmp_chars  (line 930, static)
- [x] char_composable_p  (line 991, static)
- [x] inhibit_auto_composition  (line 1008, static)
- [x] composition_compute_stop_pos  (line 1042)
- [x] composition_reseat_it  (line 1295)
- [x] composition_update_it  (line 1464)
- [x] find_automatic_composition  (line 1650)
- [x] composition_adjust_point  (line 1862)
- [x] compare_composition_rules  (line 2133, static)
- [x] mark_composite  (line 2184)
- [x] syms_of_composite  (line 2192)

## Macros defined — 3
- [x] BACKWARD_CHAR  (line 1582, body 695076ff5d7d)
- [x] FORWARD_CHAR  (line 1566, body 5e3c8db6ff7e)
- [x] MAX_AUTO_COMPOSITION_LOOKBACK  (line 156, body 77de68daecd8)

## Types and their fields — 1
- [x] position_record: ptrdiff_t pos, ptrdiff_t pos_byte, unsigned char* p

## File-scope objects — 7
- [x] struct composition composition_table  (line 141)
- [x] static ptrdiff_t composition_table_size  (line 144, static)
- [x] ptrdiff_t n_compositions  (line 147)
- [x] static Lisp_Object composition_hash_table  (line 152, static)
- [x] static Lisp_Object gstring_hash_table  (line 641, static)
- [x] static Lisp_Object gstring_work  (line 840, static)
- [x] static Lisp_Object gstring_work_headers  (line 841, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/composite.c` @ `fd7afced403`; port `femacs/src/composite.rs` (2,381 lines vs C's
2,312 live of 2,322 — the 2.98% drift is measured against the **`live`** baseline declared in
`docs/ports/line-drift-tolerance.tsv`, because the one dead arm is recorded here rather than
transcribed).  `src/composite.c` is in `docs/baseline/frozen-build-sources.txt`, so the frozen
configuration compiles it: there is no §3.31 exemption and the full comparison applies.  The
gates:

    build-tu src/composite.c      -> built src/composite.rs -> ../build/femacs/composite.o
    extract-abi.py --check        -> OK emacs/src/composite.c -> femacs/src/composite.rs
    extract-abi.py --check (.h)   -> OK emacs/src/composite.h -> femacs/src/include/composite.rs
    check-symbols.py --pending    -> check-symbols: 78 units compared, 0 mismatched, 1 still pending
    check-globals.py              -> check-globals: 243 units, 0 DEFVAR_* variables without a definition in their own unit
    ledger                        -> 43/43 rows checked, 0 unchecked

Note on the symbol gate's coverage: the frozen build has no `build/emacs-ref/src/*.o` (only
the `lib/` objects exist, status §3.31), so `check-symbols.py`'s 78 compared units are the
`lib/` ones and this `src/` unit is not among them.  Its C-visible symbol surface was checked
by hand against `nm -g --defined-only build/femacs/composite.o`: the 19 non-`static` C
functions, the two C globals (`composition_table`, `n_compositions`), the six `DEFUN` entries
and the five `DEFVAR_LISP` storages this unit owns are the only unmangled defined symbols —
no invented ones, and no C-visible name missing.  The two helper functions the header mirror
contributes (`COMPOSITION_DECODE_REFS`/`_RULE`) keep Rust linkage, as does every
`#[inline(always)]` accessor.

An extra compile with `--cfg enable_checking` (the repo's step 6) also succeeds (`rustc
--edition 2024 --crate-type=lib --emit=obj --crate-name composite --cfg enable_checking
-C panic=abort -C codegen-units=1 -C opt-level=0 build/femacs/units/composite/_root.rs`); its
only diagnostics are the pre-existing mirror warnings (`unused doc comment` in
`include/lisp.rs`, the unmirrored `ccl_spec`/`Time` FFI-safety notes), not this unit.

### The condition table: the file is verified, so the dead arm is recorded, not carried

`python3 tools/check-cond-eval.py src/composite.c` reports
`1 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and `src/composite.c` is in
`docs/baseline/config/cond-eval-verified.txt`, so P-08 applies: the port transcribes the text
the frozen configuration compiles and **records** the dead region here rather than keeping it
as `#[cfg (feature = …)]` text.  The file's single `#if` is C 959 (`#ifdef
HAVE_WINDOW_SYSTEM`) … C 969, with its `#endif` at C 970 — a constant condition inside
`autocmp_chars` (PORTING.md 7.2's first class).  `HAVE_WINDOW_SYSTEM` is `undef` in the frozen
`config.h` (macros.tsv 1647), so the block is not compiled:

* C 960-969: `struct frame *f = XFRAME (font_object); if (FRAME_WINDOW_P (f)) { font_object =
  font_range (charpos, bytepos, &to, win, face, string, ch); if (! FONT_OBJECT_P (font_object)
  || (! NILP (re) && to < limit && (fast_looking_at (re, charpos, bytepos, to, -1, string) <=
  0))) return unbind_to (count, Qnil); }`.

That is the only `#if` in the file; there is no `#if 0` and no configuration-branch `#define`,
so `extract-abi.py`'s macro-body comparison covers all three macros.

### C's uninitialised locals (PORTING.md 1.5) and the two UB-free exemptions

Every C local that C merely declares (or assigns in a branch) is zero-initialised in the port,
and each is named here by its function: `get_composition_id` (`id`, `length`, `components`,
`key`, `key_contents`, `glyph_len`, `hash_index`, `method`, `cmp`, `ch`), `find_composition`
(`val`), `run_composition_function` (`start`, `end`, `func`), `update_compositions` (`prop`,
`start`, `end`), `make_composition_value_copy` (`plist`, `val`), `composition_gstring_p`
(`header`, `i`), `composition_gstring_width` (`glyph`, `font_ascent`, `font_descent`, `x`),
`composition_gstring_adjust_zero_width` (`glyph`), `fill_gstring_body` (`font`, `code` — C's
`unsigned int code;`), `autocmp_chars` (`to`, `re`, `font_object`, `lgstring`, `len`),
`char_composable_p` (`val`), `composition_compute_stop_pos` (`start`, `end`, `c`, `prop`,
`val`, `p`, `len`, `limit`), `composition_reseat_it` (`start`, `end`, `prop`, `lgstring`,
`val`, `elt`, `direction`, `cpos`, `bpos`, `i`, `j`), `composition_update_it` (`i`, `glyph`,
`from`, `column_width` — C's `c` is already initialised by `UNINIT`), `find_automatic_composition`
(`head`, `tail`, `stop`, `fore_check_limit`, `cur`, `prev`, `c`, `w`, `check`),
`composition_adjust_point` (`i`, `beg`, `end`, `val`), `Fcomposition_get_gstring` (`gstring`,
`header`, `frompos`, `frombyte`, `topos`), `Ffind_composition_internal` (`prop`, `tail`,
`gstring`, `start`, `end`, `to`, `id`, `s`, `e`, `components`, `relative_p`, `mod_func`) and
`syms_of_composite` (`i`).

One C local keeps its declaration-without-initialiser shape instead: the `struct coding_system
*coding;` of `Fcomposition_get_gstring` (C 1923), which is assigned in the very next
statement — rustc accepts the deferred `let coding: *mut coding_system;` exactly as C's
declaration is (the `floatfns.c` `rescale_for_division` shape, PORTING.md 1.5).

### Statement shapes Rust cannot write in one line

* Post-increments/decrements are `+= 1`/`-= 1` (`i++`, `from++`, `glyph++`, `charpos++`,
  `pos--`, `p--`, `bytepos--`, `nchars++`-style updates, `return n_compositions++` — the port
  saves the old value because that is what C returns).
* C's chained assignments become one statement per assignment in C's right-to-left order:
  `to = limit = charpos + len;` (`limit` first), `head = max (backlim, 0), tail = SCHARS
  (string), stop = -1;`, and `metrics->width = metrics->lbearing = metrics->rbearing = 0;`.
* The comma operator (and an assignment) inside a condition is a block expression:
  `(val = XCAR (XCDR (plist)), CONSP (val))` in `make_composition_value_copy`, `(val =
  CHAR_TABLE_REF (…), (FIXNUMP (val) && …))` in `char_composable_p`, and `((len =
  fast_looking_at (…)) > 0)` as the `else if` condition of `autocmp_chars`.
* C's `for` loops whose update step has a side effect are `while` loops with the update at the
  end of the body in C's position (the two key-materialising loops of `get_composition_id`,
  the two rule walks, the glyph-cluster walks of `composition_gstring_width`,
  `composition_update_it` and `composition_adjust_point`, the `for (i = SBYTES (string) - 1; i
  >= 0; i--)` of `Fcomposition_get_gstring`, and `syms_of_composite`'s eight headers); the
  `for (i = 0; ; i++)` of `composition_gstring_adjust_zero_width` is a `loop` with the
  increment at the end.  Where C's `continue` jumps to the update step
  (`find_automatic_composition`'s rule walk), the port advances `val` before the `continue`.
* `do { … } while (! CHAR_HEAD_P (…))` is a `loop` with the test inverted into a `break`
  (`BACKWARD_CHAR`'s inner loop, the case-(1) rewind of `find_automatic_composition`), and the
  `while (1)` of `find_automatic_composition` — whose every exit is a `return` — is `loop`
  (both are PORTING.md 2.2 shapes).
* `goto`: `get_composition_id`'s `goto invalid_composition` and `composition_reseat_it`'s two
  `goto no_composition` become the labelled loops `'invalid_composition`/`'no_composition`
  (the `break` lands on the same trailing statements); `find_automatic_composition`'s `goto
  search_forward` (case (2), which skips the rewind block) becomes the `search_forward` flag
  that guards it.  The `goto` token is exempted in `docs/ports/macro-body-notes.tsv` (the
  `mpz_get_str`/`bool_vector_binop_driver` precedent).

### The mirror: composite.h was a WIP, and this unit completed it

`femacs/src/include/composite.rs` (469 lines) now carries the whole header:
`composition_method` and its five constants, `MAX_COMPOSITION_COMPONENTS`, the `CHECK_*`
masks, `struct composition`, the eleven `LGLYPH_IX_*`/`LGLYPH_SIZE` constants, the five INLINE
functions (`composition_registered_p`, `COMPOSITION_ENCODE_RULE_VALID`, `composition_method`,
`composition_valid_p`, `lgstring_glyph_addr`) and the accessors.  The accessors are
`#[inline(always)] pub fn`s rather than `macro_rules!` — the header's pre-existing WIP shape
for the two `character.rs` already calls (`LGSTRING_GLYPH_LEN`, `LGSTRING_GLYPH`), which the
task's "safe `fn` for value-only helpers" states as the rule; a fn also keeps C's call shape at
every use site (`COMPOSITION_ID (prop)`, no `!`), and `macro_body_of` compares its body exactly
as it compares a macro's.  Two exceptions keep C's macro form: `LGLYPH_CODE`/`LGLYPH_SET_CODE`
(`macro_rules!`, because their bodies name font.h's `FONT_INVALID_CODE`, which the composite.h
mirror cannot carry without a second declaration of a font.h name in units that also include
font.rs — a macro body is only resolved where it is expanded, i.e. in this unit), and the two
*lvalue* statement macros `COMPOSITION_DECODE_REFS`/`COMPOSITION_DECODE_RULE`
(`pub fn` helpers taking `*mut c_int`, because Rust cannot write its caller's locals,
PORTING.md 3.8; the call site passes `&raw mut rule, &raw mut gref, &raw mut nref`).  The nine `extern` prototypes
and the two objects moved into `pub mod composite_decls` + `pub mod composite_objs` with glob
imports (PORTING.md 0.1-10 and P-21, the `tempname.rs`/`thread.rs` shape): this unit *defines*
all nine functions and both objects, and a same-scope declaration would be `E0428`.
`tools/extract-abi.py --check emacs/src/composite.h femacs/src/include/composite.rs` is now
`OK` and every row of `docs/ports/include-composite.md` is ticked.

The unit's `include!` list follows C's textual order and adds the four mirrors C reaches
through headers whose own mirrors do not pull them in: `include/font.rs` (composite.h 28's
`#include "font.h"`), `../lib/include/string.rs` (lisp.h 30's `<string.h>`, for `strcmp`),
`include/itree.rs` (buffer.h 28), `include/window.rs` and `include/systime.rs` (termhooks.h
28).

### Use-site declarations, and the mirror gaps for the integrator

Two `unsafe extern "C"` blocks declare the fourteen names C gets from headers whose mirrors do
not carry them (the terminal.rs/filelock.rs shape, each with C's own prototype in a `/// C:`
line): `record_unwind_save_match_data` (lisp.h 4998), `restore_point_unwind` (lisp.h 4978),
`fast_looking_at` (lisp.h 5031-5032), `find_newline` (lisp.h 5035-5036), `tty_type_name`
(lisp.h 5375), `decode_live_terminal` (termhooks.h 961, defined in terminal.c),
`mark_object` (lisp.h 4436), `hash_remove_from_table` (lisp.h 4259), `window_point`
(window.h 1233), `get_small_narrowing_begv` (dispextern.h 3592), `qsort` (the frozen target's
`<stdlib.h>`; `lib/include/stdlib.rs` does not carry it yet),
`font_fill_lglyph_metrics` (font.h 911), `get_font_ascent_descent` (dispextern.h 3631), plus
the object `inhibit_modification_hooks` (lisp.h 3525's `DEFVAR_BOOL`; the storage is
`insdel.c`'s, this file's sibling unit, and no mirror declares it — status §3.29).

Five records the mirrors do not carry are declared at the use site for the same reason:
`struct font_metrics` (font.h 414-417, five shorts), `struct font` / `struct font_driver`
(font.h 277-404 / 589-821) and `struct composition_it` / `struct it` (dispextern.h 2386-2420 /
2422-).  For the font records only one field of each is reached
(`font->driver->encode_char (font, ch)`), so they are byte-padded prefixes and the two offsets
are **measured** with clang against the frozen configuration, not derived:
`offsetof (struct font, driver)` = 176 and `offsetof (struct font_driver, encode_char)` = 96
(the `struct composition` "measured, not derived" shape).  `XFONT_OBJECT` (the font.h 537
INLINE) and `FONT_INVALID_CODE` (font.h 585 = `0xFFFFFFFF`) are written here too.  `struct it`
carries only its first three members (`window`, `w`, `f`), whose offsets are C's 0/8/16.

**Genuine mirror gaps reported, not fixed here** (other units' mirrors keep their own
provenance): dispextern.h's `struct composition_it`/`struct it`, `get_small_narrowing_begv`
and `get_font_ascent_descent`; font.h's `struct font`/`struct font_driver`/`XFONT_OBJECT`/
`FONT_INVALID_CODE`/`font_fill_lglyph_metrics`; window.h's `window_point`; lisp.h's
`mark_object`/`record_unwind_save_match_data`/`restore_point_unwind`/`fast_looking_at`/
`find_newline`/`tty_type_name`/`hash_remove_from_table`; termhooks.h's `decode_live_terminal`;
and `<stdlib.h>`'s `qsort` in `lib/include/stdlib.rs`.  Each lands with the unit that owns the
mirror (dispextern.c/font.c/`xdisp.c`, window.c, keyboard.c/search.c, terminal.c, alloc.c, the
lib mirrors); when one of them lands, the use-site declaration here can be dropped.

### Two ABI spellings, one definition

`composition_gstring_width`'s fourth parameter is `struct font_metrics *`.  The mirror's
pre-existing WIP declaration spells it `*mut c_void` (an ABI-equal pointer, and the only other
caller, `character.rs`, passes `null_mut ()`), while this unit *defines* the function with
`*mut font_metrics` so the body keeps C's `metrics->ascent`-style tokens; the definition
shadows the glob-imported declaration and the two types are ABI-identical.  In the same way
`composition_compute_stop_pos`/`composition_reseat_it`/`composition_update_it` are declared in
the mirror with `*mut c_void` for `struct composition_it *` (this unit defines them with the
precise `*mut composition_it`), and `composition_reseat_it`'s `struct face *` is `*mut c_void`
on both sides: the value is only ever passed through (`autocmp_chars`'s use of it is inside the
dead `HAVE_WINDOW_SYSTEM` arm).

### Spelling deviations recorded rather than worked around

1. `signed char bidi_level` (C 1299) is `c_char`: the mirror's prototypes have no `c_schar`,
   and `c_char` is `i8` on every supported target, so the ABI is the same.
2. `LGLYPH_CODE`'s `TYPE_MAXIMUM (unsigned)` (composite.h 306) is gnulib's `<intprops.h>`,
   which has no mirror (P-26): the port writes `u32::MAX as u64`, and the macro's note row
   names the token (the `Fash`/`esprintf` class).
3. `composition_gstring_put_cache`'s C local is `EMACS_UINT hash` while `hash_from_key`
   returns `hash_hash_t`: the port keeps C's wider local and casts at the `hash_put` call,
   which is where C truncates implicitly.
4. The `LGLYPH_SET_*` functions take `EMACS_INT` (C's `make_fixnum` converts its argument),
   so the `fill_gstring_body` call sites pass `c as EMACS_INT` / `width as EMACS_INT`, and
   `LGLYPH_SET_CODE!` compares against the `c_uint` `FONT_INVALID_CODE`, so its call site
   passes `c as c_uint` — C's int → `unsigned int` conversion (callers must assure the value
   is not negative, as composite.h's comment says).  `COMPOSITION_GLYPH`/`COMPOSITION_RULE`
   take `ptrdiff_t` for C's `(n)`, so `composition_update_it` passes `i as isize`.
5. `struct position_record` derives `Clone`/`Copy` because C copies it by assignment
   (`prev = cur`, `check = cur`, `cur = prev`).
6. `composition_update_it`'s `int c UNINIT;` is `let mut c: c_int = 0;` — `UNINIT` is
   conf_post.h's declaration suffix (`= {0,}` in the frozen configuration), so the port writes
   the zero initialiser and the macro-name note row records the token.
7. C's implicit int↔ptrdiff_t conversions at `next_char_len`, `BYTES_BY_CHAR_HEAD`,
   `CHARACTER_WIDTH`, `CHAR_BYTES`, `COMPOSITION_GLYPH`, `LGSTRING_CHAR_LEN`, `make_fixnum`,
   `CODING_ID_NAME (coding->id)` and the `struct composition` field stores are explicit `as`
   casts (PORTING.md 2.6); `build_marker (current_buffer, …)` passes `current_buffer as *mut
   c_void` because the mirror declares that first parameter untyped (the insdel.rs shape).

### `DEFVAR_*`/`DEFSYM!` storage and names

`syms_of_composite` registers the five variables this file owns, and per status §3.29 the unit
defines them as `#[unsafe(no_mangle)] pub static mut`
(`Vcompose_chars_after_function`, `Vauto_composition_mode`, `Vauto_composition_function`,
`Vcomposition_function_table`, `Vauto_composition_emoji_eligible_codepoints`, all initialised
`Qnil`), because `include/globals.rs` only declares the `globals.f_<vname>` storage.
`check-globals.py` is the gate that sees this.  The `DEFSYM!`/`DEFVAR_LISP!` name arguments are
`c"…"` literals per status §3.38 (the two `DEFSYM`s and the five `DEFVAR_LISP`s), and the five
docstrings are C's positionally (`\n` for C's line breaks; the `syms_of_data` precedent).
`Vtext_property_default_nonsticky` is *assigned* here but owned elsewhere, so it is only read
(globals.rs's declaration), as C does.

### Macro-body notes, token families and the line budget

`docs/ports/macro-body-notes.tsv` gains ten rows for this unit: `FORWARD_CHAR` and
`BACKWARD_CHAR` (the `do { … } while (0)` tails), `get_composition_id` (`goto`),
`composition_reseat_it` (`goto`), `composition_update_it` (`uninit`),
`find_automatic_composition` (`goto`), `syms_of_composite` (`doc`, the positional docstrings),
and `Fcomposition_get_gstring` (`common_flags`, the `coding_system` bitfield the mirror packs
into `bits`).  `docs/ports/token-aliases.tsv` gains the `long_line_optimizations_p` family (C's
`current_buffer->long_line_optimizations_p` is the mirror's
`BUFFER_LONG_LINE_OPTIMIZATIONS_P`; both reads, C 1068 and 1679, are kept alive by the
occurrence check).  `docs/ports/line-drift-tolerance.tsv` gains the `composite` row (0.05,
`live`).  `docs/ports/excluded-macros.txt` needed no row: no C macro of this file is dropped.

### Changes outside the unit (for integrator review)

1. `femacs/src/include/composite.rs`: the WIP mirror completed (see above; it is this unit's
   own header mirror).
2. `docs/ports/macro-body-notes.tsv`: the ten rows above (appended).
3. `docs/ports/token-aliases.tsv`: the `long_line_optimizations_p` family (appended).
4. `docs/ports/line-drift-tolerance.tsv`: the `composite` row (appended).
5. `docs/ports/include-composite.md`: regenerated from `composite.h` and fully ticked.
6. `docs/status.md` and `docs/port-map.md` were **not** touched (their §1/§2 bookkeeping is the
   integrator's, and other workers are editing the same files this round).
