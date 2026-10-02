# emacs/src/macros.c (444 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 6
- [x] start-kbd-macro  (Fstart_kbd_macro, 1/2, doc f924528fce86, line 43)
- [x] end-kbd-macro  (Fend_kbd_macro, 0/2, doc 55ac676bcbe8, line 137)
- [x] cancel-kbd-macro-events  (Fcancel_kbd_macro_events, 0/0, doc c4616c0a9cd9, line 210)
- [x] store-kbd-macro-event  (Fstore_kbd_macro_event, 1/1, doc 13d0eaf3bf7b, line 219)
- [x] call-last-kbd-macro  (Fcall_last_kbd_macro, 0/2, doc ac036aa3aff4, line 228)
- [x] execute-kbd-macro  (Fexecute_kbd_macro, 1/3, doc 6850f1f4daba, line 276)

## Functions defined — 7
- [x] end_kbd_macro  (line 126)
- [x] store_kbd_macro_char  (line 180)
- [x] finalize_kbd_macro_chars  (line 205)
- [x] pop_kbd_macro  (line 266, static)
- [x] init_macros  (line 393)
- [x] at_end_of_macro_p  (line 403)
- [x] syms_of_macros  (line 412)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 2
- [x] EMACS_INT executing_kbd_macro_iterations  (line 33)
- [x] Lisp_Object executing_kbd_macro  (line 41)

## Decisions

- **`KVAR` is written as the field it pastes.**  keyboard.h 38 defines it as
  `((kboard)->field ## _)`, i.e. a *token-pasting* macro: `KVAR (kb, defining_kbd_macro)`
  denotes `kb->defining_kbd_macro_`.  Rust has no token pasting, so the port writes
  `(*kb).defining_kbd_macro_` at each of the four use sites (`start-kbd-macro`,
  `end-kbd-macro`'s callee, `call-last-kbd-macro`, `store_kbd_macro_char`), the macro name
  is recorded in `docs/ports/excluded-macros.txt`, and the four bodies' `kvar` exemptions
  are in `macro-body-notes.tsv` (the checker sees `KVAR` in the C body).
- **`do … while (--repeat && …)` becomes a `loop` with the decrement and the test at its
  end.**  C's `while (--repeat && (STRINGP (…) || VECTORP (…)))` decrements *before* testing
  and keeps looping while both hold, so the port puts `repeat -= 1;` and the negated
  condition immediately before `break` - same order, same number of evaluations, including
  the case where the `break` of the LOOPFUNC arm leaves the loop early.
- **Two `DEFUN!` parameters are re-bound because C's are writable.**  `end-kbd-macro`
  assigns `repeat` and `execute-kbd-macro` assigns `count`; `DEFUN!`'s pattern takes plain
  idents, so both are re-bound at the body's top level (`let mut X = X;`, the convention
  data.c's `Frem`/`Fmod`/`bool-vector-not` recorded).
- **`integer_to_intmax (XCAR (tem), &executing_kbd_macro_index)`** passes an `EMACS_INT *`
  in C, where `EMACS_INT` is `intmax_t`; the mirror's `integer_to_intmax` takes
  `*mut intmax_t` (i64) and the port's `EMACS_INT` is `isize`, so the pointer is cast - the
  two are the same type in C, which is why no value changes.
- **`Fvector (n, buf)` is already the port's call form.**  C's declaration for a MANY-arity
  subr is `Lisp_Object Fvector (ptrdiff_t, Lisp_Object *)` (lisp.h's `DEFUN_ARGS_MANY`), so
  `Fvector (kbd_macro_end - kbd_macro_buffer, kbd_macro_buffer)` passes a count and the
  array pointer; the port spells the difference as `offset_from`.
- **C's 50-line comment above `execute-kbd-macro`'s loop is carried verbatim.**  The first
  draft abbreviated it, which put the file 7.9% under C's line count and tripped the line
  drift gate; the comments are part of the port, so the block is reproduced in full.
- **`sizeof *kb->kbd_macro_buffer` is `size_of::<Lisp_Object> ()`.**  The token family in
  `token-aliases.tsv` maps `sizeof` to `size_of`, and the expression's type is the element
  type; the two identifiers it named still appear elsewhere in the same bodies.
