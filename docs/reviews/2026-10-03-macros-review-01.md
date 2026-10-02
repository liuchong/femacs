# macros round 1 — `src/macros.c` (444 lines) and `src/macros.h` (51 lines)

Second completed file pair after data.c.  `femacs/src/macros.rs` is 459 lines against C's
444 (3.4%), `femacs/src/include/macros.rs` mirrors all of macros.h.

| evidence | result |
| --- | --- |
| `extract-abi.py --check emacs/src/macros.c femacs/src/macros.rs` | **OK** |
| `extract-abi.py --check emacs/src/macros.h femacs/src/include/macros.rs` | **OK** |
| `cargo run -p xtask -- build-tu src/macros.c` | 0 errors |
| `gate-fast` / `gate-probes` | 0 / 0, `layout-check passed`, 7 units with non-frozen arms |
| `rustc --cfg enable_checking … units/macros/_root.rs` | 0 errors |
| `extract-abi.py --all` | 32 ported files: **20 done**, 12 in progress, 0 failing |

## What the round's own checks caught

1. `if (bufsize - incr < len) { … }` — the `< NAME` before a `{` parses as generic
   arguments; the condition is parenthesised (the data.c round's class).
2. `DEFUN!`'s parameters are plain idents, so `end-kbd-macro` and `execute-kbd-macro`
   re-bind `repeat`/`count` at the body's top level (C's are writable).
3. `integer_to_intmax`'s second parameter is the mirror's `intmax_t *` while the port's
   `EMACS_INT` is `isize`; the pointer is cast, which is C's own identity.
4. A first draft abbreviated C's 50-line comment above `execute-kbd-macro`'s loop, which put
   the file 7.9% under C's line count and tripped the drift gate; the block is now verbatim.
   A follow-up repair then removed eight stray C lines that the restore script had carried
   in with it - `gate-fast` failed on them (`final` is a reserved keyword) even though
   `build-tu` had not been re-run at that point, which is why the probes are re-run after
   every edit, not only after the first green build.
5. `lisp.rs` gained four prototypes macros.c calls (`message1` lisp.h 4390, `run_hook` 4781,
   `command_loop_2` 5100, `xpalloc` 5548); `xpalloc`'s `ptrdiff_t *` parameter needs a
   `/// C:` annotation for the ABI probe (P-55), the same fix `string_to_number` needed.

## Ledger decisions

`docs/ports/macros.md` records the six: `KVAR`'s token pasting written as the field it
pastes, the `do … while (--repeat && …)` shape as a `loop` with the decrement and the
negated condition before `break`, the two re-bound `DEFUN!` parameters, the
`integer_to_intmax` pointer cast, `Fvector (n, buf)` (C's MANY-arity form), the verbatim
comment block, and `sizeof`→`size_of`.  `docs/ports/include-macros.md` records that the
header is complete and that its two objects live in `pub mod macros_externs`, the shape
`include/bignum.rs` uses so the owning unit's definitions shadow the declarations
(P-21/P-39).  `macro-body-notes.tsv` gained two rows (`kvar` for the four bodies that spell
the pasted field, `doc` for `syms_of_macros`'s four DEFVAR calls).
