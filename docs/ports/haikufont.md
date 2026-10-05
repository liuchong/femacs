# emacs/src/haikufont.c (1480 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with the Haiku platform objects under
> `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s `HAVE_HAIKU` (3108-3110) - and
> `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
> `docs/baseline/frozen-build-sources.txt` lists no `haikufont.o` and `extract-abi.py`'s checklist
> for the file is empty (P-86 ②).  What follows is that clause's substitute: the **C line-range
> table**, the `## Evidence` readings and the `## Deviations`.
>
> Port: `femacs/src/haikufont.rs` (**1961 lines**), which carries the file **whole, C 1-1480** -
> no slice.  The 29 `static` functions (12 of them the `font_driver` table's methods), the one
> `struct font_driver const haikufont_driver`, the 3 `DEFUN`s, the 2 exported functions
> (`haiku_handle_font_change_event`, `syms_of_haikufont`) and the 6 file-scope objects of the
> reference stand at C's own line, in C's own order.
>
> **Acquisition note - this port is `text-only` (P-86 ③⑵).**  There is no Haiku SDK and no Haiku
> toolchain on this machine (the measurement is `docs/ports/haikuterm.md` Evidence ②, same
> machine), so there is **no arm flip on a Haiku target** and nothing here is compiled or linked
> against a Haiku system.  The `<InterfaceKit.h>`/`<SupportDefs.h>` surface this file's headers
> pull in is already mirrored by `include/haiku_support.rs`/`include/haikugui.rs` (both landed
> before this unit); the `BFont_*`/`be_*` prototypes it calls are read from `emacs/src/haiku_support.h`
> (part of the reference tree) exactly as `include/haiku_support.rs` carries them, and **every
> Haiku width, constant and enum value this port spells is `UNVERIFIED`** (P-88 ①/②).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`HAIKU_OBJ`, i.e. `HAVE_HAIKU` <- `HAVE_BE_APP`, configure.ac 3097-3114) | `feature = "have_haiku"` (on every item *and* on C 22-39's include graph, P-86 ⑧) | a Haiku app-kit configuration |
| C 1463/1465's `#ifdef USE_BE_CAIRO` (the file's **only** `#if` of its own) | `feature = "be_cairo"` (the name `femacs/xtask/Cargo.toml` registers for `USE_BE_CAIRO`), both arms carried (P-51/P-86 ①) | a Haiku configuration built with `--with-be-cairo` |
| `HAVE_WINDOW_SYSTEM`, which this file's `struct font` writes (C 843-874) and the `font_driver.draw` member needs | `feature = "have_window_system"` (set by the arm pre-flight; a Haiku configuration always has it: configure.ac 3113 sets `window_system=haiku` and 7518 then `AC_DEFINE`s it) | every Haiku configuration |

`tools/check-cond-eval.py src/haikufont.c` reads the file as
`1 conditions, 0 unknown to the evaluator, 0 disagreements [VERIFIED]`, and that one condition - the
`USE_BE_CAIRO` of C 1463 - takes its **false** arm on this target (the frozen table has it undef), so
the `Fput (Qhaiku, Qfont_driver_superseded_by, Qftcr)` of C 1464 is carried under a gate the frozen
configuration leaves off, exactly as C does.

## C line ranges

The rows are **contiguous** - each block's range also carries the blank line(s) that follow it - so
that the table covers C 1-1480 with no gaps; `check-dead-file-coverage.py` requires the last range to
reach the reference's final line.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (C 1-18) and its blank line | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 20-20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-39 | the include list: `lisp.h`, `dispextern.h`, `composite.h`, `blockinput.h`, `charset.h`, `frame.h`, `window.h`, `fontset.h`, `haikuterm.h`, `character.h`, `coding.h`, `font.h`, `termchar.h`, `pdumper.h`, `haiku_support.h`, `<math.h>`, `<stdlib.h>` | file gate |
| 40-41 | `static Lisp_Object font_cache;` | file gate |
| 42-43 | `#define METRICS_NCOLS_PER_ROW (128)` | file gate |
| 44-48 | `enum metrics_status` | file gate |
| 49-52 | `#define METRICS_STATUS (metrics)` / `METRICS_SET_STATUS (metrics, status)` | file gate |
| 53-104 | `em_charset_table` (40 records + C 103's `{ NULL }` sentinel) | file gate |
| 105-177 | `haikufont_apply_registry` (`static`) | file gate |
| 178-196 | `haikufont_get_fallback_entity` (`static`) | file gate |
| 197-202 | `haikufont_get_cache` (`static`) | file gate |
| 203-237 | `haikufont_weight_to_lisp` (`static`; C 207 the `switch (weight)`) | file gate |
| 238-274 | `haikufont_lisp_to_weight` (`static`) | file gate |
| 275-291 | `haikufont_slant_to_lisp` (`static`; C 279 the `switch (slant)`) | file gate |
| 292-306 | `haikufont_lisp_to_slant` (`static`) | file gate |
| 307-336 | `haikufont_width_to_lisp` (`static`; C 311 the `switch (width)`) | file gate |
| 337-359 | `haikufont_lisp_to_width` (`static`) | file gate |
| 360-379 | `haikufont_maybe_handle_special_family` (`static`) | file gate |
| 380-445 | `haikufont_pattern_to_entity` (`static`) | file gate |
| 446-503 | `haikufont_pattern_from_object` (`static`) | file gate |
| 504-647 | `haikufont_spec_or_entity_to_pattern` (`static`; C 562-614 the `:script` branch with its four loops, C 573/583 over the vector and C 595/606 over the list) | file gate |
| 648-657 | `haikufont_done_with_query_pattern` (`static`) | file gate |
| 658-676 | `haikufont_match` (`static`) | file gate |
| 677-709 | `haikufont_list` (`static`) | file gate |
| 710-738 | `haiku_bulk_encode` (`static`) | file gate |
| 739-754 | `haikufont_encode_char` (`static`) | file gate |
| 755-881 | `haikufont_open` (`static`) | file gate |
| 882-920 | `haikufont_close` (`static`) | file gate |
| 921-926 | `haikufont_prepare_face` (`static`, C's own empty body) | file gate |
| 927-979 | `haikufont_glyph_extents` (`static`) | file gate |
| 980-1011 | `haikufont_text_extents` (`static`) | file gate |
| 1012-1090 | `haikufont_shape` (`static`; C 1042/1058-1060 the three `alloca` calls) | file gate |
| 1091-1170 | `haikufont_draw` (`static`; C 1152 the `alloca`) | file gate |
| 1171-1198 | `haikufont_list_family` (`static`) | file gate |
| 1199-1209 | `haikufont_booleans` / `haikufont_non_booleans` | file gate |
| 1210-1216 | `haikufont_filter_properties` (`static`) | file gate |
| 1217-1234 | `struct font_driver const haikufont_driver` | file gate |
| 1235-1240 | `haikufont_should_quit_popup` (`static`) | file gate |
| 1241-1328 | `x-select-font` (`DEFUN`) | file gate |
| 1329-1367 | `font-get-system-normal-font` (`DEFUN`) | file gate |
| 1368-1406 | `font-get-system-font` (`DEFUN`) | file gate |
| 1407-1420 | `haiku_handle_font_change_event` | file gate |
| 1421-1426 | `syms_of_haikufont_for_pdumper` (`static`) | file gate |
| 1427-1480 | `syms_of_haikufont`; C 1459-1461 the `DEFVAR_BOOL`, C 1463-1465 the file's one `#ifdef USE_BE_CAIRO` | file gate (C 1463-1465: `be_cairo`) |

## Evidence

**① The frozen reading (P-86 ⑧'s judgement: the file gate must reach the include graph).**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/hkfont-lane cargo run -q -p xtask -- build-tu src/haikufont.c
built src/haikufont.rs -> ../build/femacs/haikufont.o
exit 0
$ ls -l build/femacs/haikufont.o
-rw-r--r--@ 1 liu  staff  336 Oct  5 17:32 build/femacs/haikufont.o
$ md5 build/femacs/haikufont.o
MD5 (build/femacs/haikufont.o) = dbed68837694579185377dbb0821ab62
```

**exit 0** and a **336-byte empty object** - `include/config.rs` is the only item the frozen
configuration compiles, and every one of C 22-39's includes and every item this file declares sits
behind `have_haiku`.  (Placing the include edges outside the gate is what makes the frozen compile
fail: P-86 ⑧'s `E0428 Emacs_Cursor`/`cannot find macro` measurement.)

**② The arm reading.**  `xtask probe` with the file gate and the one gate a Haiku configuration
always has:

```
$ cd femacs && RUSTUP_TOOLCHAIN=nightly-2026-10-01 CARGO_TARGET_DIR=/tmp/hkfont-lane \
    cargo run -q -p xtask -- probe src/haikufont.rs /tmp/hkfont-lane/haikufont-arm.rlib \
    --crate-type=lib -C panic=abort \
    --cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"'
exit 1
```

- **errors: 1**, and it is a **mirror copy**, not the unit:
  `build/probes/haikufont/_Users_…_femacs_src_include_systime_rs.rs:28` = `femacs/src/include/systime.rs`
  28's `const _: () = core::assert! (!cfg! (feature = "have_haiku") || HAIKU);` (`E0080`, `HAIKU` is
  undefined) - the item `docs/ports/haikuterm.md` Owed 6 and `docs/ports/haikuselect.md` Owed 5 name.
- **errors in the unit itself (`_root.rs`): 0**, and **warnings in the unit: 0** (the 5 warnings are
  all mirror copies: `include/dispextern.rs` 692, `lib/include/stdbit.in.rs` 644,
  `include/lisp.rs` 3940, `include/charset.rs` 698 `unused doc comment`, `include/frame.rs` 1360
  `unused attribute`; none is in this unit's statements).
- toolchain: `rustc 1.101.0-nightly (21b707e3f 2026-09-30)`.  Nightly is needed for the same reason
  as the sibling units: `include/lisp.rs` 3761's `verror` is a `c_variadic` declaration, and this
  unit calls `error` (C 1264), which is the same form.  The two `--cfg`s are the file gate plus the
  window-system gate; without the second, the two `Emacs_Cursor` definitions
  (`include/dispextern.rs` 105 and `include/haikugui.rs` 60) collide in the crate root (measured).
- **The readings on the way to that one** (recorded because they are what this unit's own error list
  looked like before each fix): the first `build-tu` run - which parses every item whatever its
  `#[cfg]` - reported **8** (`_root.rs` syntax: two unbraced `if` bodies, two cast-`<` parses, one
  `DEFUN!` without the file gate); the three `probe` readings before the last two reported **20**
  (`_root.rs` types: the `METRICS_STATUS` `i16`, the `s->face` cast, the two `*mut c_void`
  parameters of `haiku_merge_cursor_foreground`/`haiku_draw_background_rect`, three driver methods
  with the Rust ABI, `be_select_font`'s two `Option<…>` arguments, three non-exhaustive `match`es,
  the mirror's `()`-returning `be_lock_font_defaults`, five E0381s), **6** (five E0381s) and **1**
  (the mirror's).  **No error in any of them was a defect of the reference text**, and every
  `_root.rs` error named this unit's own line numbers - which is also the non-vacuity evidence ③
  elaborates.

**③ The mutation control (P-86 ⑦: the reading above is not vacuous).**  Injecting one type error
into the unit's own text - `femacs/src/haikufont.rs`'s `let mut b_len: ptrdiff_t = 0;` (C 1151)
replaced by `= c"injected_control";` - changes the reading from **1 error to 2**, and the new one
names the injected line:

```
error[E0308]: mismatched types
    --> .../build/probes/haikufont/_root.rs:1537:34
1537 |       let mut b_len: ptrdiff_t = c"injected_control";             // C 1151
     |                      ---------   ^^^^^^^^^^^^^^^^^^^ expected `isize`, found `&CStr`
```

`_root.rs`'s line numbering is the unit's own (every `include!` is replaced by one line), so the
error is the unit's line 1537 and not a mirror's.  Restored, and `shasum -a 256 femacs/src/haikufont.rs`
→ `515014b7e2ecf538179aeec6cd194f0f1cec2135bd11a8351e1c0e900c33ab45` before and after the injection
(that is the revision the injection ran on).  Readings ①/②/⑤ below were then re-taken on the final
revision `733e6cbe05ae00adb1c56de41160453b4c17e244f58afa21147fe90fe13bb817`: `diff` between the two
revisions is **three comment-only hunks** (C 1218-1234's citations, `femacs/src/haikufont.rs`
1618-1621; the `config.h` line's own `// C 19` → `// C 20`; and the header's `C 21-39` → `C 22-39`)
and nothing else, and `build/femacs/haikufont.o`'s md5 is identical across them - so no reading here
straddles a code change.

**④ `extract-abi.py --check`.**

```
$ python3 tools/extract-abi.py --check emacs/src/haikufont.c femacs/src/haikufont.rs
- function haikufont_weight_to_lisp does not mention case, switch
- function haikufont_slant_to_lisp does not mention case, switch
- function haikufont_width_to_lisp does not mention case, switch
- function haikufont_shape does not mention alloca
- function haikufont_draw does not mention alloca, font_info
- function syms_of_haikufont does not mention doc
- line count drifted 32.5%: C 1480 vs Rust 1961
FAIL emacs/src/haikufont.c -> femacs/src/haikufont.rs
```

**One row is the line-count row** (C 1480 → Rust 1961 = 32.5%), and its tolerance row is the
integrator's (`## Owed` 3).  The six `does not mention …` rows are the recorded spelling differences
of `## Deviations` 5/6/9/10 and now sit in `docs/ports/macro-body-notes.tsv` (appended, `## Owed` 4);
each is a *declaration of a spelling difference*, not proof that the bodies match - the body
comparison itself is this same reading.

**⑤ The five text gates (whole-tree last lines; whether this lane's files appear).**

| gate | whole-tree last line | this lane's files |
| --- | --- | --- |
| `check-declarations.py` | `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures` | no finding |
| `check-mirror-dupes.py` | `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)` | no finding |
| `check-dead-file-coverage.py` | `125 ledger(s) bound (a C range table), 0 failure(s)` | this ledger is bound to `src/haikufont.c`: `info docs/ports/haikufont.md: 44 range row(s) cover src/haikufont.c (1480 lines; 0 blank line(s) between blocks need no row)` |
| `check-ledger-ticks.py` | `234 ledger(s) bound (a function tick to compare), 0 failure(s), 0 carried by another file, 356 row(s) with no function tick …, 0 row(s) without a ledger` | no finding (this ledger has no `- [x]` ticks) |
| `check-citations.py` | `676 file(s) scanned, 0 drift, 0 range, 1604 external, 36 allowed, 0 stale` | no finding |

`check-citations.py` first read **1 drift** against this unit -
`drift femacs/src/haikufont.rs:1620 font.h 682-698` with the snippet `harfbuzz` ("is at
font.h:74") - because the
`have_window_system` citation's line also named the harfbuzz pair: the comment now carries both
citations (`font.h 682-698` for `draw`/`get_bitmap`/`free_bitmap`, `font.h 807-820` for the harfbuzz
pair), and the gate reads `0 drift` on the fixed text.  That is a real finding this lane fixed, not a
pre-existing one.

The five whole-tree readings were taken with **other lanes' files landing in the working tree while
this lane ran** (`femacs/src/gmalloc.rs`, `femacs/src/pgtkselect.rs`, `femacs/oldXMenu/*.rs`,
`docs/ports/macro-body-notes.tsv`'s later rows): the bound/external counts move with those lanes'
ledgers, citations and ratchet rows (an earlier read of the same five gates, on the same tree a few
commits' worth of edits ago, came out `18 declaration sites (1 forbidden, …), 1 failures` /
`21 duplicate declaration(s), …, 4 failure(s)` / `116 ledger(s) bound …` / `233 ledger(s) bound …` /
`1522 external …`).  On the final tree **four of the five gates end `0 failure(s)`**, and the earlier
failing pair carried no `haikufont` mention at all (`grep -c haikufont` on that run's output: 0 -
its findings were `femacs/src/gmalloc.rs` and `femacs/oldXMenu/insque.rs`, both other lanes' files);
the only gate that reports a row for this file is `extract-abi --check`, and that row is the
line-count row of ④.

This lane's command budget: `build-tu` **4** (the run that found the parse errors, the frozen reading
on the fixed text, the reading after the first comment fix, and the reading on the final revision) -
one over the named budget of 3; `probe` **6** (three fix-driven readings, the mutant, the clean
re-read, and the re-read on the final revision) - twice the named budget of 3.  Both are reported as
they are.  `cargo test`/`gate-fast`/`xtask --all`/`xtask dead-arms`/`check-all-units.sh`
not run; git untouched (no `add`/`commit`/`push`/`stash`/`checkout`).

## Deviations

1. **The licence/version block (C 1-18) is dropped** and the port opens with the single
   `// femacs: port of src/haikufont.c @ fd7afced403.` line (PORTING.md 0.1-19).
2. **C 22-39's include list is written as the port's `include!` list** (PORTING.md 0.1-1..4), on the
   file gate; `haiku_support.h` (C 36) is reached through `haikuterm.h` (C 30) first, as in C, and
   this entry is C's own second, guarded reach.
3. **Seventeen names the frozen mirrors of this graph do not carry are declared at their use site**
   (P-72 rule 4, each with one `/// C:` line and the header it was read from):
   `strncpy` (`<string.h>`), `decode_window_system_frame` (frame.h 1475), `request_sigio`/
   `unrequest_sigio` (dispextern.h 3781/3782), `prepare_face_for_display` (dispextern.h 3810),
   `quit` (lisp.h 4786), `process_pending_signals` (lisp.h 3880), `font_make_entity` (font.h 842),
   `font_make_object` (font.h 846), `font_style_to_value` (font.h 854), `font_style_symbolic`
   (font.h 856), `register_font_driver` (font.h 896) and `font_filter_properties` (font.h 927).
   `include/font.rs` carries the `FONT_SET_STYLE`/`FONT_*` macro bodies that name the two
   `font_style_*` functions but declares neither, so a unit compiled on its own owes them (the
   `src/xfaces.rs` 526-529 precedent).
4. **`struct face` (dispextern.h 1787-1937) is this unit's use-site record** (P-72 rule 3):
   `include/dispextern.rs`'s copy is behind `dispextern_face_records`.  C reads `foreground`
   (C 1108) and `background` (C 1109) through it, so those two members are pinned at
   `docs/baseline/face-layout.tsv`'s measured window-system offsets (`foreground`@184,
   `background`@192, `sizeof` 296, align 8; the frozen arm is 280/8 with `foreground`@168, and the
   sixteen bytes of C 1797-1811's `gc`/`stipple` pair are the only difference) - the **Haiku arm
   itself is `UNVERIFIED`** here, since no Haiku SDK is on this machine, and `gc` is a pointer in
   every arm.
5. **C 1042/1058-1060 and C 1152's bare `alloca` calls are the port's stack pool** `sa_stack_alloc`
   (PORTING.md P-05/3.2, the `src/hbfont.rs` 1310 and `src/w32font.rs` 1478 shape).  C's own
   `alloca` frees at function return; the pool block is released by the next `safe_free` at a
   shallower depth (the `alloc.rs` P-05 accounting).
6. **C 1098's `struct font_info *info = (struct font_info *) s->font`** is C's own type-pun: the
   only read through it is C 1142's `((struct haikufont_info *) info)->be_font`, so the port types
   the local `*mut c_void` and casts at that use site (`(*(info as *mut haikufont_info)).be_font`).
   `struct font_info` here is *not* `ftfont.h` 56's record - C never reads a field of it.
7. **`haikufont_info.metrics` is `*mut *mut c_void` in the mirror** (`include/haikuterm.rs` 46, the
   P-72 (3) untyped member: `struct font_metrics` is `include/font.rs`'s record, which landed after
   that mirror).  `haikufont_glyph_extents` therefore casts twice - C 960's `info->metrics[row] + col`
   and the five C 970-974 stores through `cache` - while `xrealloc`/`memset`/`xfree` and the
   `info->metrics[i]` reads keep C's pointer arithmetic, which the element size makes identical.
   `## Owed` 1 names the mirror round that owes the typed member.
8. **C 1463-1465's `#ifdef USE_BE_CAIRO` arm is carried under `feature = "be_cairo"`** (P-51/P-86
   ①), the arm this target does not take.
9. **The three `switch` statements (C 207, C 279, C 311) are `match` statements** with one arm per
   `case` and the `_ => { }` arm an `int`-typed match needs for C's absent `default:` (PORTING.md
   2.2/P-04; the `src/font.rs` 946 shape).  The four `if`-chains (C 239, C 293, C 338, C 361) keep C's
   own statements.  `case`, `switch` and `default` are the three tokens this spelling drops
   (`docs/ports/macro-body-notes.tsv`).
10. **C's clause in the `DEFUN`/`DEFVAR_BOOL` forms is a macro argument**: the port's `DEFUN!`
    takes the docstring as a string literal and `DEFVAR_BOOL!` takes it as its third positional
    argument, so the `doc:` field label does not appear (the `syms_of_haikuselect` row class).
11. **C's implicit conversions are spelled with `as` casts**: `int`/`ptrdiff_t`/`size_t` comparisons
    and indices, `unsigned`→`unsigned short` truncations (C 724/734), `int`→`short` stores
    (C 970-974, 996-998, 1000, 1002, 1010), `bool`→`int` stores (C 499, 547, 641) and C 1292's
    `bool rc` → the port's `as c_int`.  Each keeps C's own conversion direction.
12. **C's uninitialized locals are initialized** (E0381): `be_font` (C 762), `length` (C 1176),
    `advance`/`lb`/`rb` (C 967), `foreground`/`background` (C 1101), and the two
    `struct haiku_font_pattern` locals (C 664/682/760 and the two `DEFUN`s), which
    `haikufont_spec_or_entity_to_pattern` fills field by field - the port states the zero value
    C's own declaration starts from (PORTING.md 1.5).
13. **C 1132-1134's chained assignment `height = ascent = <expr>` is two assignments** in C's own
    order (`ascent = <expr>; height = ascent;`): a Rust assignment is `()`.
14. **C 1051's `b = xrealloc (b, b_len = (b_len + slen))` keeps the assignment inside the call**
    as a block expression, so `b_len` is still written exactly once, before the call reads it.
15. **`be_lock_font_defaults` is called through a unit-local module** (`haiku_support_abi`): the
    mirror `include/haiku_support.rs` 731 declares it with **no return type** while
    `haiku_support.h` 769 declares `extern bool be_lock_font_defaults (void);`, and C 1341/1380
    read that value.  A second crate-root item of the same name beside the mirror's would be an
    `E0428`, so the unit declares the same symbol (`#[link_name]`) with the reference's prototype
    inside a module and calls it from there (the `src/haikuselect.rs` 188-193 `block_input_abi`
    shape; `## Owed` 2).
16. **Two cross-record members are written as C's own value**: `(*font).driver`
    is `*const c_void` in `include/font.rs` (P-72 (3)), so C 844's `&haikufont_driver` is
    `&raw const haikufont_driver as *const c_void`, and `vertical_centering` is the mirror's `bool`
    for C's `bool_bf` bit (C 848/872, PORTING.md P-80).
17. **`(*pattern).family`/`style`/`(*ptn).style` writes go through `as_mut_ptr ()`** where C writes
    `(char *) &pattern->family`, and `sizeof pattern->family - 1` is
    `std::mem::size_of::<haiku_font_family_or_style> () - 1` (63): the mirror's
    `haiku_font_family_or_style` is `[c_char; 64]`, so the two spellings are the same address and
    the same bound.
18. **The Haiku handle's truthiness tests and scalars**: the port keeps C's `if (!x)` tests on
    `info->be_font` (`(*info).be_font.is_null ()`) and C's own truncations where the handle's
    return values are stored (`unichars[i] = idx`, `unichars[i] = 0xFFFF`), which are C's
    `unsigned short` conversions of `unsigned int` (C 724/734).

## Owed

1. **`include/haikuterm.rs` 46's `haikufont_info.metrics` is `*mut *mut c_void`**, while
   `struct font_metrics` has since landed in `include/font.rs` (haikuterm.h 46 spells the member
   `struct font_metrics **metrics`).  Owner: `docs/ports/include-haikuterm.md` - the mirror should
   name the record, and this unit's two casts (`## Deviations` 7) then go away.
2. **`include/haiku_support.rs` 731's `be_lock_font_defaults` is declared with no return type**,
   while `haiku_support.h` 769 declares `extern bool be_lock_font_defaults (void);`.  Owner:
   `docs/ports/include-haiku_support.md` (or its ledger) - the mirror needs the `bool` (`## Deviations`
   15), after which this unit's `haiku_support_abi` module is deleted.
3. `docs/ports/line-drift-tolerance.tsv`: one row, `haikufont`, tolerance >= 0.325, basis `total`
   (C 1480 → Rust 1961, `## Evidence` ④).  Not in this lane's writable set.
4. **`docs/ports/macro-body-notes.tsv`'s six new rows** (appended by this lane, `## Evidence` ④).
   They are declarations of a spelling difference; the body comparison itself is
   `extract-abi --check`'s reading, which stays red until item 3 lands.
5. **A Haiku round** (P-86 ③⑴, needs the SDK this machine has not got): confirm `be_open_font_at_index`'s
   `float` parameter, `BFont_metrics`'s eleven `int *` outputs, `BFont_nchar_bounds`'s `int32_t`
   length, the `haiku_font_*` enum values, and this unit's `struct face` offsets
   (`## Deviations` 4) - all `UNVERIFIED` here.
6. **`docs/baseline/port-lines-pending.tsv`'s row is now stale**: the tree reads
   `stale    src/haikufont.c: no longer under the threshold` (1,961 Rust lines for 1,480 C lines is
   133%).  `python3 tools/check-port-lines.py --write-pending` is the integrator's step, and the
   `docs/port-map.md` row's target count is regenerated with it.
7. **`include/systime.rs` 28's `HAIKU` assertion** stays the arm probe's one error, so no Haiku
   configuration can compile a whole crate until the constant lands.  Owner:
   `docs/ports/include-systime.md` (the same item `docs/ports/haikuterm.md` Owed 6 and
   `docs/ports/haikuselect.md` Owed 5 name).
8. This ledger's `## C line ranges` table is machine-read by `check-dead-file-coverage.py`; a future
   slice that moves an item must keep the table ascending and complete.

## Not claimed

- **No Haiku-target compile or link.**  There is no Haiku SDK or toolchain on this machine, so
  Evidence ②'s reading is a `/tmp`/`build`-tree harness reading on this host, not a Haiku build;
  nothing here runs.
- **No runtime, font, or rendering behaviour.**  The font cache, the fallback entity, the pattern
  matching, the glyph encoding, the extents, the shaping and the drawing are transcribed against
  their Haiku entry points, not exercised - the `BFont_*`/`be_*` implementations are the Haiku
  C++ half (`haiku_font_support.cc`), which is out of reach here.
- **Every Haiku width, constant, enum value and prototype in this file is C's spelling, not a
  measurement** and is `UNVERIFIED`; `## Owed` 5 is the round that measures them.
- **The `struct face` offsets are another record's measurement**, not this unit's (`## Deviations` 4).
- **`xtask gate-fast`, `check-all-units.sh`, `cargo test`, `xtask dead-arms` and
  `extract-abi --all` were not run** (this lane's budget names them as out of scope).
