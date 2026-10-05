# emacs/src/syntax.c (3851 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 21
- [x] syntax-table-p  (Fsyntax_table_p, 1/1, doc f715f7de9e0a, line 984)
- [x] syntax-table  (Fsyntax_table, 0/0, doc 9c85093ac9cf, line 1002)
- [x] standard-syntax-table  (Fstandard_syntax_table, 0/0, doc 9abf7991b7ee, line 1010)
- [x] copy-syntax-table  (Fcopy_syntax_table, 0/1, doc a73b308f53cf, line 1019)
- [x] set-syntax-table  (Fset_syntax_table, 1/1, doc acef725ad145, line 1045)
- [x] char-syntax  (Fchar_syntax, 1/1, doc ee705559d656, line 1098)
- [x] syntax-class-to-char  (Fsyntax_class_to_char, 1/1, doc d0295bd86d69, line 1124)
- [x] matching-paren  (Fmatching_paren, 1/1, doc 6c58716a57c0, line 1141)
- [x] string-to-syntax  (Fstring_to_syntax, 1/1, doc 7798ca190dd4, line 1156)
- [x] modify-syntax-entry  (Fmodify_syntax_entry, 2/3, doc 949126f6844c, line 1234)
- [x] internal-describe-syntax-value  (Finternal_describe_syntax_value, 1/1, doc 32e16c545dc6, line 1305)
- [x] forward-word  (Fforward_word, 0/1, doc 479b1b66765b, line 1581)
- [x] skip-chars-forward  (Fskip_chars_forward, 1/2, doc 2323ebabe4ef, line 1618)
- [x] skip-chars-backward  (Fskip_chars_backward, 1/2, doc 7b4dc48de405, line 1633)
- [x] skip-syntax-forward  (Fskip_syntax_forward, 1/2, doc 0c459083d7ce, line 1642)
- [x] skip-syntax-backward  (Fskip_syntax_backward, 1/2, doc 6ab49b64a64c, line 1653)
- [x] forward-comment  (Fforward_comment, 1/1, doc 79e8e7d5490c, line 2436)
- [x] scan-lists  (Fscan_lists, 3/3, doc 660afc9dca98, line 3049)
- [x] scan-sexps  (Fscan_sexps, 2/2, doc 33b6b3032f9c, line 3077)
- [x] backward-prefix-chars  (Fbackward_prefix_chars, 0/0, doc e14e2d5a2384, line 3096)
- [x] parse-partial-sexp  (Fparse_partial_sexp, 2/6, doc 8be7f570e614, line 3567)

## Functions defined — 40
- [x] SYNTAX_FLAGS_COMSTART_FIRST  (line 62, static)
- [x] SYNTAX_FLAGS_COMSTART_SECOND  (line 67, static)
- [x] SYNTAX_FLAGS_COMEND_FIRST  (line 72, static)
- [x] SYNTAX_FLAGS_COMEND_SECOND  (line 77, static)
- [x] SYNTAX_FLAGS_COMSTARTEND_FIRST  (line 82, static)
- [x] SYNTAX_FLAGS_PREFIX  (line 87, static)
- [x] SYNTAX_FLAGS_COMMENT_STYLEB  (line 92, static)
- [x] SYNTAX_FLAGS_COMMENT_STYLEC  (line 97, static)
- [x] SYNTAX_FLAGS_COMMENT_STYLEC2  (line 102, static)
- [x] SYNTAX_FLAGS_COMMENT_NESTED  (line 107, static)
- [x] SYNTAX_FLAGS_COMMENT_STYLE  (line 115, static)
- [x] SYNTAX_COMEND_FIRST  (line 125, static)
- [x] bset_syntax_table  (line 193, static)
- [x] syntax_prefix_flag_p  (line 200)
- [x] SET_RAW_SYNTAX_ENTRY  (line 213, static)
- [x] SET_RAW_SYNTAX_ENTRY_RANGE  (line 222, static)
- [x] SYNTAX_MATCH  (line 232, static)
- [x] SETUP_SYNTAX_TABLE  (line 250, static)
- [x] RE_SETUP_SYNTAX_TABLE_FOR_OBJECT  (line 277)
- [x] update_syntax_table  (line 328)
- [x] parse_sexp_propertize  (line 480, static)
- [x] update_syntax_table_forward  (line 513)
- [x] char_quoted  (line 534, static)
- [x] dec_bytepos  (line 563, static)
- [x] find_defun_start  (line 585, static)
- [x] prev_char_comend_first  (line 669, static)
- [x] back_comment  (line 695, static)
- [x] check_syntax_table  (line 996, static)
- [x] scan_words  (line 1464)
- [x] skip_chars  (line 1666, static)
- [x] skip_syntaxes  (line 2112, static)
- [x] in_classes  (line 2275, static)
- [x] forw_comment  (line 2312, static)
- [x] syntax_multibyte  (line 2654, static)
- [x] scan_lists  (line 2660, static)
- [x] in_2char_comment_start  (line 3145, static)
- [x] scan_sexps_forward  (line 3179, static)
- [x] internalize_parse_state  (line 3503, static)
- [x] init_syntax_once  (line 3664)
- [x] syms_of_syntax  (line 3747)

## Macros defined — 5
- [x] INC_FROM  (line 3212, body 70f51f2835af)
- [x] SYNTAX  (line 32, body 61d123e5ded5)
- [x] SYNTAX_ENTRY  (line 33, body b58e6d1b901d)
- [x] SYNTAX_WITH_FLAGS  (line 34, body a28d1ae79592)
- [x] uninitialized_interval  (line 239, body e5ba6798356e)

## Types and their fields — 1
- [x] lisp_parse_state: EMACS_INT depth, int instring, EMACS_INT incomment, int comstyle, bool quoted, EMACS_INT mindepth, ptrdiff_t thislevelstart, ptrdiff_t prevlevelstart, ptrdiff_t location, ptrdiff_t location_byte, ptrdiff_t comstr_start, Lisp_Object levelstarts, int prev_syntax

## File-scope objects — 7
- [x] static ptrdiff_t find_start_pos  (line 173, static)
- [x] static ptrdiff_t find_start_value  (line 174, static)
- [x] static ptrdiff_t find_start_value_byte  (line 175, static)
- [x] static struct buffer find_start_buffer  (line 176, static)
- [x] static ptrdiff_t find_start_begv  (line 177, static)
- [x] static modiff_count find_start_modiff  (line 178, static)
- [x] static Lisp_Object Vsyntax_code_object  (line 1095, static)

## Decisions

Round 59: **complete.**  `femacs/src/syntax.rs` carries the whole file (C 1-3851): every `static`
helper, every macro, every subr, `init_syntax_once`, `scan_lists`, `scan_sexps_forward`,
`internalize_parse_state` and `syms_of_syntax`.  `build-tu src/syntax.c` builds
(`../build/femacs/syntax.o`, 0 errors) and `extract-abi.py --check emacs/src/syntax.c
femacs/src/syntax.rs` reports **OK**.

Round 58 left C 2660-3851 unwritten; this round added it in C's order - `scan_lists` (C 2660),
`INC_FROM` (C 3212), `scan_sexps_forward` (C 3179), `internalize_parse_state` (C 3503), the four subrs
`scan-lists`/`scan-sexps`/`backward-prefix-chars`/`parse-partial-sexp` and `syms_of_syntax` (C 3747)
with its 19 `defsubr`s - plus the 24 `docs/ports/macro-body-notes.tsv` rows the token check needs and
one `docs/ports/line-drift-tolerance.tsv` row (0.12).

Two defects in `tools/extract-abi.py` were found *by* this file and fixed, because each of them made
the gate blind rather than merely noisy:

* `block_body` counted braces inside character literals.  The port writes C's `'{' as c_int` and
  `'}' as c_int` directly, and the `'}'` one closed the body early, so `init_syntax_once` compared as
  a *truncated* body: every token after it (`char_table_set_range`, `MAX_CHAR`, `0x80`) was invisible
  to the containment check.  Character literals are now skipped, which also hardens the C side.
* Integer literals were compared by spelling.  C's octal constant has no legal Rust spelling
  (`0200` is not a Rust literal), so a port that writes `0o200` was reported as missing `0200`, and
  the `memcpy` family's argument check demanded C's `0200`.  `body_token_seq` and
  `family_call_numbers` now normalise every integer literal (C octal, `0o`/`0b`/`0x`, decimal) to its
  decimal value, which also stops `0x80` from tokenising as `0` + `x80` (that made `0x80` and `128`
  compare unequal).  Regression: the eleven already-OK units (cmds, terminal, cm, scroll, doc,
  casefiddle, character, region-cache, tempname, qcopy-acl, regcomp) still report OK.

### The one semantic trap this file carries

**C 30-32 redefine syntax.h's three macros with `VIA_PROPERTY = 1`**:
`#define SYNTAX(c) syntax_property (c, 1)`, and likewise `SYNTAX_ENTRY` and `SYNTAX_WITH_FLAGS`.
syntax.h's own versions pass `false`, and those are what `include/syntax.rs` keeps for the *other*
units.  Inside syntax.c every `SYNTAX (c)` therefore reads `gl_state`, the interval-provided table,
instead of the buffer's table.  A port that used the mirror's functions would be wrong in
`char_quoted`, `back_comment`, `scan_words`, `forw_comment`, `scan_lists` and
`Finternal_describe_syntax_value` while every gate stayed green, because a token-set check cannot see
`true` where C writes `1`.  The unit defines the three names itself, and the mirror moved its own
versions into `syntax_h_own_definitions` behind a glob import so that the unit's definitions shadow
them (PORTING.md P-39/P-21).

### Mirrors this file needed, and what they cost

* `include/intervals.rs` is **new**: syntax.c is the first unit that reads interval fields
  (`i->position`, `i->plist`) and C's `LENGTH` / `INTERVAL_LAST_POS` arithmetic, so intervals.h had to
  have a home before this file could compile.  `struct interval`'s layout is *measured*, not guessed:
  clang with the spikes/run.sh flags reports sizeof 56 / align 8 and offsets total_length 0,
  position 8, left 16, right 24, up 32, plist 48, with the six `bool_bf : 1` members as one byte at
  40.  A standalone `rustc` compile of the mirror plus ten compile-time assertions (`size_of`,
  `align_of`, `offset_of`, and `size_of::<interval_up>`) passes, so those offsets are proved rather
  than asserted in prose.  Adding `interval` to `tools/gen-layout-fields.py`'s STRUCTS, so the layout
  gate asserts it too, is left to the round that owns the layout baseline.

* `include/regex-emacs.rs` is **new** and partial: `re_wctype_t`, its 18 `RECC_*` constants,
  `re_iswctype` and `re_wctype_parse`.  `RECC_NUM_CLASSES` repeats `RECC_UNIBYTE`'s value, which a
  Rust enum cannot spell, so the type is `c_int` with one constant per C name - the same shape
  `include/syntax.rs` uses for `syntaxcode`.

* `include/syntax.rs` gained one `pub mod syntax_h_own_definitions` block (`gl_state`,
  `syntax_spec_code`, `syntax_prefix_flag_p`, `scan_words`, `update_syntax_table`,
  `update_syntax_table_forward`, `RE_SETUP_SYNTAX_TABLE_FOR_OBJECT`, and the three redefined macros)
  plus one glob import: every name inside it is defined by this unit.  Other consumers such as
  `casefiddle.rs` and `cmds.rs` see exactly the names they saw before, through that import.

* `include/lisp.rs` gained four declarations that lisp.h owns and no mirror had yet: `insert`,
  `insert_char`, `insert_string`, `insert1` (lisp.h 4338-4339, 4919) and `scan_newline` (lisp.h 5037).

### Constructs, and the C text they stand for

* `syntax_spec_code[0400]` (C 1063): C names 128 of the 256 entries and zero-fills the rest, so the
  port spells all 256, `0o377` for C's `0377` and `Swhitespace as u8` for the named ones.

* `goto` uses the three shapes PORTING.md 2.2 allows, each with its C line at the site: a labelled
  block (`'found`, `'done`, `'dispatch`, `'fwd_ok` and the other `*_ok` labels), a loop label where
  C's `break` must leave the loop from inside such a block (`'comment`, `'fwd`, `'fwd_unibyte`,
  `'back`, `'back_unibyte`), and one entry-stage flag where C jumps *into* a loop body
  (`forw_comment`'s `goto forw_incomment`).  `back_comment`'s two jumps became one flag each, because
  `goto lossage` runs a block that `goto done` skips.

* `switch` becomes `match`; C's `FALLTHROUGH` is one shared arm
  (`Sstring_fence | Scomment_fence | Sstring`) or a fall into the next statement with the C comment
  kept in place (`Scomment_fence` into `Scomment`).

* Locals C leaves uninitialised are zero-initialised and marked - `comstart_byte`,
  `out_charpos`/`out_bytepos`, and `char_ranges`'s `UNINIT` - per PORTING.md 0.1-13.

* The file's own helpers that dereference the buffer globals are `unsafe fn`, the cmds.rs shape,
  because C's macros expand to `(*current_buffer)` derefs.  Elsewhere `unsafe` appears only where C
  has a pointer, a bitfield or a union (PORTING.md P-79).

### Gate limits this file exposes

* The goto/switch spellings of `char_quoted`, `find_defun_start`, `back_comment`, `skip_chars`,
  `skip_syntaxes`, `forw_comment`, `scan_lists`, `scan_sexps_forward`, `Fstring_to_syntax`,
  `Finternal_describe_syntax_value` and `Fforward_comment` are recorded as 24
  `docs/ports/macro-body-notes.tsv` rows, one family per row, each naming what the port's shape is
  (`match` arms, merged `FALLTHROUGH` arms, labelled blocks, `jumped_to_*` flags, entry stages).
  `SYNTAX`, `SYNTAX_ENTRY` and `SYNTAX_WITH_FLAGS` differ on `1` versus `true` (the casetab
  precedent), the four `Fskip_*` subrs, `Fset_syntax_table`, `Fscan_lists`/`Fscan_sexps` and
  `syms_of_syntax` differ the same way on C's `0`/`1` booleans, and `char_quoted`/`INC_FROM` on
  `bool quoted = 0` and the `do { … } while (0)` idiom.  None of them changes behaviour.

* The octal report defect this file used to expose (C's `0200` read as decimal 200, so
  `skip_chars`'s `0400`/`0200` and the `memcpy` family row read as missing) is **fixed** in
  `tools/extract-abi.py`: integer literals are normalised by value on both sides, so the notes rows
  no longer need to cover the octal spelling at all.  `uninitialized_interval`'s cast is likewise
  spelled `1 as INTERVAL`, so it compares token-for-token.

* The layout gate cannot see bitfields, as for every bitfield mirror; for `struct interval` the
  byte-level proof is the assertion set named above.

### Round 59: the shapes the new half uses

* `scan_lists` keeps C's four labels as one labelled block around both scans (`'lose`, C's `lose:`)
  plus one labelled block per scan for C's `done:`/`done2:` (the jump means "the object ended, count
  one and resume"), and C's `close1:`/`open2:` bodies - reachable both by `FALLTHROUGH` from `Smath`
  and by `goto` - run once after the `match`, selected by the `jumped_to_close1`/`jumped_to_open2`
  flag the arms set.

* `scan_sexps_forward` enters its loop at six places and leaves it at three.  The six entries are the
  loop's `STAGE_*` stages (`STAGE_ATCOMMENT`, `STAGE_STARTINCOMMENT`, `STAGE_STARTQUOTED`,
  `STAGE_SYMSTARTED`, `STAGE_STARTINSTRING`, `STAGE_STARTQUOTEDINSTRING`), so each C label keeps its
  own arm and no code is duplicated; the three exits are `EXIT_DONE`/`EXIT_STOP`/`EXIT_ENDQUOTED`,
  which select C's `stop:`/`endquoted:` epilogues before the shared `done:` tail.  `symdone:` is the
  `'symloop` label's tail; `startquotedinstring` is the flag the string-scan arm re-enters with.

* `INC_FROM` (C 3212) reads and writes this function's locals, so it is a `macro_rules!` defined
  inside the body (the scroll.rs `CHECK_BOUNDS` precedent, PORTING.md P-80's macro rules); its
  `do { … } while (0)` is one block, which is the `0` the notes row records.

* `TYPE_MINIMUM`/`TYPE_MAXIMUM` are unit-local `macro_rules!` over the type's own limits (the
  lib/dtotimespec.rs shape), and `back_comment`'s C range test now spells `TYPE_MINIMUM! (EMACS_INT)`
  instead of `EMACS_INT::MIN`, so that call no longer needs a notes token.

* C's function-local `struct level` becomes a nested `#[repr(C)] struct level { last, prev }` in the
  same field order, and `curlevel`/`endlevel` are real raw pointers, because C compares and advances
  them (`curlevel != levelstart`, `(curlevel - 1)->last`).

* `scan_words`'s two `while (true)` scans are `while true` in the port (C's keyword, kept), and the
  two `ch0`/`ch1` locals C leaves uninitialised are zero-initialised with the PORTING.md 0.1-13
  marker, because a `while true` body is not known to run at least once.

* `update_redisplay_ticks` (dispextern.h 4052) has no mirror yet, so it is declared at its single use
  site inside `scan_sexps_forward` (the doc.rs/casefiddle.rs shape).

## Owed (2026-10-06, status 3.634): the interval cache is NULL where C's callers never leave it

With `tools/loadup-fixture.sh` the loadup reaches `progmodes/elisp-mode.el` (100 `Loading` lines) and
stops in `update_syntax_table` (syntax.rs 406, C 361): `Error in syntax_table logic for to-the-end
intervals`.  That `error` fires only when the *cached* interval is NULL (`i = count > 0 ?
gl_state.forward_i : gl_state.backward_i`).  The sentinel (`uninitialized_interval`, syntax.rs 282 /
syntax.c 239) and the four `parse_sexp_lookup_properties` sites match C one for one, so the divergence
is in the callers that drive the cache (`scan-sexps`/`scan_lists`/`SETUP_SYNTAX_TABLE` family) or in
the interval data itself - not in this function.  Not localized yet; recorded so the next round starts
here rather than re-deriving it.
