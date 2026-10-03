# emacs/src/cmds.c (538 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 7
- [x] forward-char  (Fforward_char, 0/1, doc 2b646069b030, line 69)
- [x] backward-char  (Fbackward_char, 0/1, doc e9d70374d745, line 83)
- [x] forward-line  (Fforward_line, 0/1, doc dead6a31e5cf, line 97)
- [x] beginning-of-line  (Fbeginning_of_line, 0/1, doc b81ad104b75b, line 148)
- [x] end-of-line  (Fend_of_line, 0/1, doc 9a203480fb79, line 172)
- [x] delete-char  (Fdelete_char, 1/2, doc 862efc1e22dc, line 221)
- [x] self-insert-command  (Fself_insert_command, 1/2, doc 42195652df15, line 263)

## Functions defined — 3
- [x] move_point  (line 37, static)
- [x] internal_self_insert  (line 313, static)
- [x] syms_of_cmds  (line 507)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **C's forward declaration of `internal_self_insert` (C 32) is deleted** — C's "prototype then
  definition" pair has no Rust counterpart (PORTING.md 0.1-12); the definition stays where C
  defines it (C 313).
- **Uninitialised locals get C's zero value first.**  `forward-line`'s `ptrdiff_t pos, pos_byte`
  are filled through `scan_newline_from_point`'s out-parameters; Rust cannot borrow a
  not-yet-initialised local, so they start at 0 and are assigned by the call, in C's order
  (PORTING.md 0.1-13).  `new_point`, `pos`, `count`, `newpos`, `to` are declared then assigned
  exactly where C assigns them.
- **C's `bool` parameters that the body assigns to** (`end-of-line`, `self-insert-command`,
  `beginning-of-line`'s `n`) are shadowed with a `let mut` copy, because `DEFUN!` binds its
  arguments immutably and C assigns to them (`n = make_fixnum (1);`, `c = last_command_event;`).
- **Assignment inside a condition** (`(cwidth = XFIXNAT (Fchar_width (…))) != 0`,
  C 371) becomes a block expression in the `else if` condition, which keeps C's "assign, then
  test the assigned value" order.
- **`ckd_add (&to, PT, chars_to_delete)`** is the `overflowing_add` pair the timespec-add.rs
  round established (status §3.x / PORTING.md P-15): the sum is stored either way, and the
  boolean decides the `PTRDIFF_MAX` clamp.
- **`%"pI"d` is written `%td`** in the `error` call (lisp.h 150's `pI` is `"t"`), the
  region-cache.rs spelling; the constant itself lives in `lisp.rs`.
- **`Vpost_self_insert_hook` is defined here**, because cmds.c owns it in C (`DEFVAR_LISP`'s
  `&globals.f_Vpost_self_insert_hook`); the port's `V*` names are `globals.rs`'s extern
  declarations, so the unit that owns one must also define it — the casetab.rs/character.rs
  shape (status §3.29).
- **The twenty notes rows' family:** `Fforward_char`/`Fbackward_char` pass `true`/`false` where
  C passes `1`/`0` to `bool forward` (PORTING.md 5.2, the `Fset_case_table` precedent), and
  `syms_of_cmds`'s `DEFVAR_LISP` takes the docstring positionally, so C's `doc:` label has no
  counterpart.  All three are in `docs/ports/macro-body-notes.tsv`.
- **`move_point`'s `XFIXNUM (n)` reads and the `min`/`eabs` calls** are the mirrors' own shapes
  (`min` is `isize`, `eabs!` is a macro that evaluates its argument twice when negative, exactly
  as C's does).
