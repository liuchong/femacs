# emacs/src/minibuf.c (2759 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 23
- [x] active-minibuffer-window  (Factive_minibuffer_window, 0/0, doc 0a5c4e59d0c0, line 215)
- [x] set-minibuffer-window  (Fset_minibuffer_window, 1/1, doc e36e1e6ffa59, line 241)
- [x] minibufferp  (Fminibufferp, 0/2, doc f5529f1cb85c, line 403)
- [x] innermost-minibuffer-p  (Finnermost_minibuffer_p, 0/1, doc b076cc05f4ec, line 424)
- [x] minibuffer-innermost-command-loop-p  (Fminibuffer_innermost_command_loop_p, 0/1, doc 8eba7fad33eb, line 438)
- [x] abort-minibuffers  (Fabort_minibuffers, 0/0, doc 690ea6520db3, line 472)
- [x] minibuffer-prompt-end  (Fminibuffer_prompt_end, 0/0, doc 325c0cfd70e2, line 505)
- [x] minibuffer-contents  (Fminibuffer_contents, 0/0, doc cd4c6e683664, line 527)
- [x] minibuffer-contents-no-properties  (Fminibuffer_contents_no_properties, 0/0, doc 806c33be522f, line 537)
- [x] read-from-minibuffer  (Fread_from_minibuffer, 1/7, doc 6a7bbaf14228, line 1309)
- [x] read-string  (Fread_string, 1/5, doc 1195534e65e1, line 1420)
- [x] read-command  (Fread_command, 1/2, doc f46a3d89288b, line 1462)
- [x] read-variable  (Fread_variable, 1/2, doc ec98c5cc4960, line 1496)
- [x] read-buffer  (Fread_buffer, 1/4, doc 7dce689e28d0, line 1522)
- [x] try-completion  (Ftry_completion, 2/3, doc d37dc3dddd30, line 1615)
- [x] all-completions  (Fall_completions, 2/3, doc f6d4ef69d92b, line 1846)
- [x] completing-read  (Fcompleting_read, 2/8, doc 12ec42e7df7f, line 1986)
- [x] test-completion  (Ftest_completion, 2/3, doc a0ab7d03a764, line 2062)
- [x] internal-complete-buffer  (Finternal_complete_buffer, 3/3, doc 60105765103f, line 2156)
- [x] assoc-string  (Fassoc_string, 2/3, doc 715b68bb36d8, line 2203)
- [x] minibuffer-depth  (Fminibuffer_depth, 0/0, doc ce9ac83c6726, line 2241)
- [x] minibuffer-prompt  (Fminibuffer_prompt, 0/0, doc d3b86c6533e6, line 2248)
- [x] completion--flex-cost-gotoh  (Fcompletion__flex_cost_gotoh, 2/2, doc a75d267cb411, line 2345)

## Functions defined — 24
- [x] minibuf_follows_frame  (line 86, static)
- [x] minibuf_moves_frame_when_opened  (line 105, static)
- [x] choose_minibuf_frame  (line 115, static)
- [x] minibuffer_ent_greater  (line 137, static)
- [x] zip_minibuffer_stacks  (line 150, static)
- [x] move_minibuffers_onto_frame  (line 195)
- [x] string_to_object  (line 269, static)
- [x] read_minibuf_noninteractive  (line 308, static)
- [x] live_minibuffer_p  (line 388, static)
- [x] this_minibuffer_depth  (line 457)
- [x] read_minibuf  (line 573, static)
- [x] is_minibuffer  (line 996)
- [x] nth_minibuffer  (line 1006, static)
- [x] set_minibuffer_mode  (line 1016, static)
- [x] get_minibuffer  (line 1042)
- [x] run_exit_minibuf_hook  (line 1089, static)
- [x] read_minibuf_unwind  (line 1111, static)
- [x] minibuffer_unwind  (line 1264, static)
- [x] barf_if_interaction_inhibited  (line 1303)
- [x] match_regexps  (line 1595, static)
- [x] set_initial_minibuffer_mode  (line 2259)
- [x] init_minibuf_once  (line 2268)
- [x] init_minibuf_once_for_pdumper  (line 2278, static)
- [x] syms_of_minibuf  (line 2493)

## Macros defined — 4
- [x] FLEX_MAX_MATRIX_SIZE  (line 2357, body 225f3d8e4819)
- [x] FLEX_MAX_PAT_SIZE  (line 2356, body b4182bff4b3c)
- [x] FLEX_MAX_STR_SIZE  (line 2355, body ce09b127d48f)
- [x] MAT  (line 2359, body b2905a2b881b)

## Types and their fields — 0

## File-scope objects — 9
- [x] Lisp_Object Vminibuffer_list  (line 47)
- [x] static Lisp_Object Vcommand_loop_level_list  (line 48, static)
- [x] static Lisp_Object minibuf_save_list  (line 52, static)
- [x] EMACS_INT minibuf_level  (line 56)
- [x] Lisp_Object last_minibuf_string  (line 60)
- [x] static Lisp_Object minibuf_prompt  (line 64, static)
- [x] static Lisp_Object MB_frame  (line 70, static)
- [x] static ptrdiff_t minibuf_prompt_width  (line 75, static)
- [x] static Lisp_Object exp_MB_frame  (line 1105, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/minibuf.c` @ `fd7afced403`; port `femacs/src/minibuf.rs` (2,779 lines vs C's 2,759
total / 2,748 live — +0.7% on the **total** basis and +1.1% on the `live` one, both inside
the 5% the rule allows every file, so **no row was added to
`docs/ports/line-drift-tolerance.tsv`**; the per-function token comparison is the real
guard and it is `OK`).  `src/minibuf.c` is in `docs/baseline/frozen-build-sources.txt`, so
the frozen configuration compiles it: there is no 3.31 exemption and the full comparison
applies.  The gates:

    build-tu src/minibuf.c        -> built src/minibuf.rs -> ../build/femacs/minibuf.o
    extract-abi.py --check        -> OK emacs/src/minibuf.c -> femacs/src/minibuf.rs
    check-symbols.py --pending    -> check-symbols: 78 units compared, 0 mismatched, 1 still pending
    check-globals.py              -> check-globals: 251 units, 0 DEFVAR_* variables without a definition in their own unit
    ledger                        -> 60/60 rows checked, 0 unchecked
    (extra) rustc --cfg enable_checking on the materialized unit -> exit 0, no error

An extra compile with `--cfg enable_checking` (the repo's step 6) also succeeds
(`rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C codegen-units=1
-C opt-level=0 --cfg enable_checking build/femacs/units/minibuf/_root.rs`, run **from
`femacs/`** so rustup applies the pinned toolchain — the bare `rustc` at the repo root picks
the default toolchain and fails inside `lisp.rs`'s `verror` with E0658, `c_variadic`, a
pre-existing mirror property, not this unit's).  The only diagnostics are the two
pre-existing mirror warnings (`include/lisp.rs` 3836's unused doc comment and
`include/frame.rs`'s duplicated `#[inline]`); neither is this unit's.

- **What the file carries.** C's 23 live `DEFUN`s (`read-function`, C 1486-1493, is in the
  dead `#ifdef NOTDEF` arm and has no counterpart), its 24 functions, its four macros
  (`FLEX_MAX_STR_SIZE`, `FLEX_MAX_PAT_SIZE`, `FLEX_MAX_MATRIX_SIZE`, `MAT`), the nine
  file-scope objects, and 25 `DEFVAR_*` storages (§3.29).  The eight non-`static` functions
  and three non-`static` objects carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the five
  `static` functions whose address C takes (`minibuffer_ent_greater` to `merge_c`,
  `run_exit_minibuf_hook`, `read_minibuf_unwind`, `minibuffer_unwind`,
  `init_minibuf_once_for_pdumper`) are `unsafe extern "C" fn` (PORTING.md 1.4); the rest of
  the `static` helpers are plain `unsafe fn`.
- **`minibuf_c_loop_level` is not in this checklist, and no gate covers it.**  C 1080
  defines it (`static EMACS_INT minibuf_c_loop_level (EMACS_INT depth)`, called by
  `Fminibuffer_innermost_command_loop_p`), but `tools/cinventory.py`'s function scan does
  not list it (C 78's forward declaration of the same name is the likely reason — the same
  quirk does not hide `nth_minibuffer`/`set_minibuffer_mode`, which have forward
  declarations too).  The port writes it at its C position, with C's own statements, and
  neither the ledger nor `--check`'s body-token rule sees it: it is verified only by reading
  it against C 1080-1086.
- **Dead arms recorded, not transcribed (P-08).** `src/minibuf.c` is in
  `docs/baseline/config/cond-eval-verified.txt` and `python3 tools/check-cond-eval.py
  src/minibuf.c` reports `7 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the
  frozen configuration's branch selection is clang's.  The seven arms are:
  * C 37-39 `#ifdef HAVE_NTGUI` — an `#include "w32term.h"`, so it has no Rust text at all
    (`HAVE_NTGUI` is `undef`, macros.tsv 1451);
  * C 92-100, the literal `#if 0`, which is the whole `minibuf_stays_put` function; dead
    text, so it has **no Rust counterpart at all**;
  * C 607-610 `#ifdef HAVE_WINDOW_SYSTEM` (`if (display_hourglass_p) cancel_hourglass ();`)
    — `HAVE_WINDOW_SYSTEM` is `undef` (macros.tsv 1647), so the live text of that region is
    the following `if (!NILP (initial))`, which the port writes;
  * C 933-943, the `#ifndef HAVE_NTGUI` pair: the live arm is
    `flush_frame (XFRAME (XWINDOW (minibuf_window)->frame))` and the dead `#else` arm is
    `w32_flip_buffers_if_dirty`'s comment plus call;
  * C 1173-1177, the literal `#if 0` inside `read_minibuf_unwind` (the `temp`/
    `minibuf_window` restore, whose two statements are never compiled);
  * C 1396-1408 `#ifdef HAVE_TEXT_CONVERSION` (the `overriding-text-conversion-style` reset
    in `Fread_from_minibuffer`) — `undef` (macros.tsv 1614);
  * C 1485-1494 `#ifdef NOTDEF`, the `read-function` DEFUN (`NOTDEF` is never defined; the
    macro table does not name it, and clang's answer is the same).
  None of them is a structural condition, so no `#[cfg (feature = …)]` text is carried and
  the ledger's `live`-basis row would apply (it is not needed: the drift is inside 5% on
  both bases).
- **Use-site declarations (mirror gaps).**  Each is written with C's own prototype in a
  `/// C:` line; nothing under `femacs/src/include/` or `femacs/lib/include/` was touched, so
  no other consumer needed rebuilding:
  * `lisp.h` 4799 `FUNCTIONP`, 4266 `merge_c` (its `bool (*)(Lisp_Object, Lisp_Object)`
    parameter is `Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> bool>`, P-27),
    4271 `nconc2`, 4825 `record_unwind_protect_void`, 5097 `safe_run_hooks`, 5102
    `recursive_edit_1`, 5119 `do_switch_frame`, 5120 `get_frame_param`, 4388
    `clear_message`, 4566 `make_specified_string`.
  * `lisp.h` 2504-2506 `DOOBARRAY`, written as `macro_rules! DOOBARRAY` whose last argument
    is the body (the mirror carries `obarray_iter_t` and its four helpers); the `for` is the
    one token with no counterpart.
  * `window.h` 1127 `restore_window_configuration`, 1128 `restore_focus_frame`, 1139
    `set_window_buffer`, and the `WINDOW_FRAME` macro (543), written as `macro_rules!`.
  * `dispextern.h` 3570 `resize_mini_window`, 3897 `update_frame`, 4012 `is_tty_frame`,
    4017 `combine_updates_for_frame`, and **`struct redisplay_interface`** (3110-3200):
    `frame.h` 1913-1920's `INLINE void flush_frame` is that struct's only consumer here, and
    the port writes C's own body, with the ten members C declares before `flush_display`
    mirrored as an opaque nine-pointer prefix and `offsetof (flush_display) == 72` pinned by
    clang against the frozen target.  `sizeof` is **not** pinned (the record is a prefix by
    construction) — this is the one partial record this unit declares.
  * `frame.h`'s `FRAME_RIF` (`(f)->terminal->rif`) is reached through termhooks.rs's
    `terminal.rif`, which is P-72③'s `*mut c_void`, so the port casts it back the way
    P-55/P-72 rule ③ prescribe.
  * `<stdio.h>`: `fflush`, `getchar`, `putchar`, `putc` and `stdout` (an object-like macro
    on this target: `#[cfg_attr (target_os = "macos", link_name = "__stdoutp")]`, the cm.rs
    shape).  `<unistd.h>`: `STDIN_FILENO` (a measured constant).  `systty.h` **has a mirror
    since 2026-10-04** (`femacs/src/include/systty.rs`, which brings the system mirror
    `lib/include/termios.rs`): this unit now gets `struct emacs_tty` (75-85), the real
    `struct termios` inside it, `CDISABLE` and the three tty declarations (87-89) from there,
    and the opaque `struct termios { _storage: [u8; 72] }` stand-in this unit carried until
    then - a record that could not be read field by field - is gone.
  * `fd_set`-style includes are not needed; the four extra mirrors this unit includes
    (`itree.rs`, `systime.rs`, `dispextern.rs`, `coding.rs`, `composite.rs`, `termhooks.rs`)
    are the ones `buffer.rs`, `keyboard.rs` and `frame.rs` reach through their own C
    headers' includes (C 28/29/30), in cmds.rs's proven order.
- **Spelling deviations, one by one** (`docs/ports/macro-body-notes.tsv` carries the nine
  token-level rows C's bodies need):
  1. C's `BVAR (b, field)` is `((b)->field ## _)`, but the port's `BVAR!` does not append
     the underscore, so the four fields this unit reaches are spelled with the port's own
     names: `directory_`, `enable_multibyte_characters_`, `name_`, `case_canon_table_`.
  2. `BEG`/`BEG_BYTE` are `c_int` constants in the port (not the place-macros `BEGV`/`PT`/`Z`
     are), so C's `make_fixnum (BEG)` is `make_fixnum (BEG as EMACS_INT)` and `PT > BEG` is
     `PT! () > BEG as ptrdiff_t` (the insdel.rs/callproc.rs spelling).
  3. Object-like `Q*` symbols are functions in the port (P-17), so each use carries `()`;
     `XSETFASTINT`/`XSETINT`/`XSETCAR`/`XSETCDR`/`XSETSYMBOL` keep C's macro names and
     `DEFSYM`/`DEFVAR_*` become `DEFSYM!`/`DEFVAR_LISP!`/`DEFVAR_BOOL!`.
  4. C's local `enum { function_table, … } type` is the file-scope
     `try_completion_type`, read through the raw identifier `r#type` (a Rust keyword).
     `Fall_completions`' C `int type` stays `c_int` with C's `0`/`1`/`2`/`3` literals.
  5. `min`/`max` are lisp.rs's `isize` functions, so the flex body's `int` operands are cast
     (`min (… as isize, … as isize) as c_int`); every cast is value-preserving for the
     bounded costs the algorithm uses.
  6. C's three `#define`s `FLEX_MAX_STR_SIZE`/`FLEX_MAX_PAT_SIZE`/`FLEX_MAX_MATRIX_SIZE` are
     Rust constants (the extractor's constant rule requires the names, and the third keeps
     C's own expression); `MAT` is a `macro_rules!` defined **inside** the body, after
     `width`, because a `macro_rules!` resolves the locals it mentions at its definition
     site and C's `#define` sits before them.  C's `((i) + 1) * width` promotes `int` to
     `ptrdiff_t`; the port writes that promotion and the `usize` index conversion explicitly.
     C's `typedef struct iter { … } iter_t` is the port's `struct iter` plus `type iter_t =
     iter` so both C names have a counterpart.
  7. Window/tty bits: `XWINDOW (…)->must_be_updated_p = true` and `->suspend_auto_hscroll =
     0` are `bits |= 1 << WINDOW_MUST_BE_UPDATED_P_SHIFT` and `bits &= ! (1 << …)`
     (PORTING.md 1.3, the insdel.rs shape).
  8. `Fminibufferp`'s C `return (COND) ? Qt : Qnil;` needs its condition in a `let` (`found`,
     a port-only local) before the `if`/`else` expression: Rust has no ternary and an
     `if`-expression cannot be nested directly under another `if` head.
  9. `++`/`--` statements are `+= 1`/`-= 1`; C's `for (…; …; i++)` loops over a counter are
     `while` loops with the increment at the body's end (or `for i in …` where the bound is
     a plain `usize` — none here needs that); C's comma-expressions in conditions
     (`(tem = Fcompare_strings (…), EQ (Qt, tem))`) are blocks that assign and then yield the
     test.
  10. Object-like C spellings that are macros in the reference: `UNINIT` (a plain
      zero-initialised `bool`, PORTING.md 1.5), `INT_STRLEN_BOUND` (one `NAME_BUF_SIZE`
      constant with the json.rs/data.rs derivation), `INT_MAX` (limits.in.rs's constant),
      `IS_DAEMON`/`DAEMON_RUNNING` (lisp.rs's functions), `STRING_MULTIBYTE` and
      `SSDATA`/`SDATA`/`SREF` (mirror accessors), `FRAME_SELECTED_WINDOW`,
      `FOR_EACH_FRAME` (the mirror's `!`-macro with the body as last argument),
      `FOR_EACH_LIVE_BUFFER` (the open P-22 loop macro, expanded by hand with C's `tail`/`buf`
      names, the unlock_all_files row), `WINDOW_FRAME`/`DOOBARRAY` (below).
  11. `struct emacs_tty etty` and the other C locals that C declares uninitialised are
      zero-initialised (`std::mem::zeroed ()` / `= 0` / `= Qnil ()`), and the parameters C
      assigns to are shadowed `let mut x = x;` (PORTING.md 1.5, 0.1-13).
  12. C's forward declarations (C 77-80, 260-262, 2265) are dropped (PORTING.md 0.1-12).
- **Call-relationship differences (D7).  None found.**  Every call in the C text appears in
  the port with the same callee and the same argument order, and the port adds no call the
  reference does not make; the differences are the ones above (a spelling, a degraded
  parameter type, or a macro that expands at the same point C's does).
- **What no gate can see.**  (a) The four `*mut c_void`-class degradations this unit inherits
  (`terminal.rif`, `window.current_matrix`/`desired_matrix` are not touched here; `rif` is)
  and the partial `struct redisplay_interface` prefix — only the pinned `flush_display`
  offset is checked.  (b) `minibuf_c_loop_level` has no ledger row (above).  (c) The
  undefined-reference check the repo prescribes for exactly this class of defect
  (status 3.57): `nm -u build/femacs/minibuf.o` was read, and every one of its 170 names
  resolves to a reference declaration except the 14 Rust runtime symbols
  (`core::panicking::*`, `core::option::unwrap_failed`) and `__stdoutp`, which is libc's
  `stdout` on this target and is what cm.rs already links against.
