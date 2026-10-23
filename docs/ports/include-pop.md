# emacs/lib-src/pop.h (75 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 5
- [x] GETLINE_INCR  (line 26, body 128351137a9c)
- [x] GETLINE_MIN  (line 24, body 128351137a9c)
- [x] POP_NO_GETPASS  (line 49, body d04e8274b40c)
- [x] POP_NO_HESIOD  (line 48, body d0768c568695)
- [x] POP_NO_KERBEROS  (line 47, body 98cf78881eac)

## Types and their fields — 1
- [x] _popserver: int file, int data, char* buffer, int buffer_size, int buffer_index, bool_bf in_multi : 1, bool_bf trash_started : 1

## File-scope objects — 2
- [x] extern char pop_error  (line 29)
- [x] extern bool pop_debug  (line 30)

## Decisions

Mirror of lib-src/pop.h, complete: five macros, two extern objects, the `_popserver` record with
its two `bool_bf` bitfields, the `popserver` typedef and eighteen prototypes - the ledger's eight
boxes are ticked.  Its consumers are live: `lib-src/movemail.c` (in the reference
`lib-src/Makefile`'s `UTILITIES`) and `lib-src/pop.c` itself, which is linked into movemail.

1. **Layout, measured.**  The `struct _popserver` probe (clang against the frozen configuration:
   `-I build/emacs-ref/src -I emacs/src`) prints
   `sizeof=32 file=0 data=4 buffer=8 buffer_size=16 buffer_index=20 bits=1`: the two `bool_bf`
   members - `conf_post.h` 61's `typedef bool bool_bf;`, one byte - occupy offset 24, so the
   mirror keeps them as `bits: u8` with `popserver_bits!` naming `in_multi` and `trash_started`
   (PORTING.md 1.3/P-80, the coding.rs shape).
2. **`pop_error` is an incomplete array in the header.**  C declares `extern char pop_error[];`
   and pop.c defines the size (`ERROR_MAX` 160); the mirror uses the zero-length stand-in the
   port reserves for that shape (`lisp.rs`'s `my_edata`/`my_endbss`), and the unit that defines
   it writes the real 160.
3. **The prototypes live in `pop_decls` + glob import** (P-21): lib-src/pop.c defines every one
   of them, so the defining unit's definitions shadow the declarations.
4. **Nothing compiles this mirror yet.**  `xtask layout-check` compiles `src/include/*.rs` and
   whatever they include; a `lib-src/` mirror is reached only through a lib-src unit, and both of
   its consumers are still unported, so the assertion block above has run only in this file's own
   `--check`.  When movemail.c or pop.c lands, their `build-tu` becomes the compile proof.

Evidence: `python3 tools/extract-abi.py --check emacs/lib-src/pop.h
lib-src/include/pop.rs` -> `OK` (8/8 boxes).
