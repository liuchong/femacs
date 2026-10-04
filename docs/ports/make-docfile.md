# emacs/lib-src/make-docfile.c (1232 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 21
- [x] verror  (line 86, static)
- [x] error  (line 96, static)
- [x] fatal  (line 107, static)
- [x] memory_exhausted  (line 117, static)
- [x] xmalloc  (line 125, static)
- [x] xrealloc  (line 136, static)
- [x] main  (line 146)
- [x] put_filename  (line 220, static)
- [x] scan_file  (line 237, static)
- [x] start_globals  (line 245, static)
- [x] put_char  (line 284, static)
- [x] scan_keyword_or_put_char  (line 321, static)
- [x] read_c_string_or_comment  (line 401, static)
- [x] write_c_args  (line 490, static)
- [x] add_global  (line 603, static)
- [x] compare_globals  (line 639, static)
- [x] close_emacs_globals  (line 672, static)
- [x] write_globals  (line 685, static)
- [x] scan_c_file  (line 804, static)
- [x] stream_match  (line 838, static)
- [x] scan_c_stream  (line 852, static)

## Macros defined — 0

## Types and their fields — 2
- [x] global: enum global_type type, char* name, int flags, int value, char const* svalue, <aggregate> v
- [x] rcsoc_state: intmax_t pending_spaces, intmax_t pending_newlines, FILE* in_file, char* buf_ptr, char* buf_lim, FILE* out_file, const char* keyword, const char* cur_keyword_ptr, bool saw_keyword

## File-scope objects — 6
- [x] static char progname  (line 78, static)
- [x] static bool generate_globals  (line 81, static)
- [x] static char input_buffer  (line 252, static)
- [x] static ptrdiff_t num_globals  (line 598, static)
- [x] static ptrdiff_t num_globals_allocated  (line 599, static)
- [x] static struct global globals  (line 600, static)

## Decisions

- **Shape.** One unit, `femacs/lib-src/make-docfile.rs`, written in the `lib-src/hexl.rs` /
  `lib-src/make-fingerprint.rs` shape: `#![allow (…)]`, `use std::ffi::{…}`, the `include!`
  list in C's own order, then C's objects/functions in C's order, `main` as
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char)
  -> c_int` (C's `main` is the only non-`static` definition, so it is the only `no_mangle`
  name; the 20 helpers stay `unsafe fn`/`unsafe extern "C" fn` with internal linkage).
- **Types.** `struct rcsoc_state` and `struct global` are `#[repr(C)]` with C's field names
  (`type` is the Rust `r#type`), and the anonymous `union { int value; char const *svalue; }`
  is the named `#[repr(C)] union global_v` (Rust needs the type).  `enum global_type` is a
  `#[repr(C)]` enum (C's enum ABI is `int`) and the anonymous `DEFUN_*` enum is four `const`s.
  Both records are file-local (no header baseline holds them), so their layout is *measured*:
  a clang probe compiled the same declarations against the frozen headers and printed
  `global 32 {type 0, name 8, flags 16, v 24}` and `rcsoc_state 72 {pending_spaces 0,
  pending_newlines 8, in_file 16, buf_ptr 24, buf_lim 32, out_file 40, keyword 48,
  cur_keyword_ptr 56, saw_keyword 64}`; the unit carries those as `const _` `size_of`/
  `offset_of!` assertions.
- **Dead arms (P-08/P-51).** `WINDOWSNT` and `DOS_NT` are `undef` in the frozen
  configuration, so C 53-58 (`#undef fopen` + `<direct.h>`) and C 60-67 (`#undef chdir`) are
  preprocessor-only arms that carry no compiled statement; they are recorded here, not
  transcribed (the `lib-src/movemail.rs` precedent).  Evidence:
  `python3 tools/check-cond-eval.py lib-src/make-docfile.c` ->
  `2 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file's only two conditions).
- **Use-site declarations (P-26/P-72 (3)).** `freopen`, `ungetc`, `fscanf`, `vfprintf`,
  `qsort`, `strstr` and `exit` have no mirror (or no mirror entry), so each is declared at
  the unit with a `/// C:` line (`qsort`'s comparison parameter is
  `unsafe extern "C" fn (…)`, and `compare_globals` is defined with that ABI, PORTING.md
  1.4).  `ptrdiff_t` and `intmax_t` are the frozen target's `isize`/`i64` written out (this
  program has no `lisp.h` mirror); `DIRECTORY_SEP`/`IS_DIRECTORY_SEP` are generated
  `<config.h>` 120/2097, which the config mirror does not carry (the `src/dired.rs`
  precedent).  `<stdarg.h>`, `<intprops.h>` and `<unlocked-io.h>` have no mirror and are
  recorded in the file header (the `...` parameter *is* the list, P-04).
- **Deviations recorded.**
  1. C 69-73's five forward declarations are dropped: Rust items are order-independent and a
     declaration plus a definition is `E0428` (PORTING.md 0.1-12).
  2. `struct rcsoc_state state;` (C 405) is zeroed first, then every C assignment is kept:
     Rust has no partial initialization (0.1-13/1.5).  The same rule gives `commas`,
     `minargs`, `maxargs` (C 854) their zero initializers.
  3. `char *ident_start UNINIT;` (C 494): `UNINIT` is empty in the frozen configuration, so
     the port writes the null pointer it stands for (P-26's ledger class; the
     `cons_to_unsigned`/`skip_chars` precedent).
  4. C's `for (…; …; p++)` loops whose bodies `continue` are written as `while` with an
     explicit increment before each `continue` (`put_filename` has none, `write_c_args` and
     the other scanners do), so the C semantics (the increment runs on `continue`) is kept.
  5. `switch` becomes `match` (PORTING.md 2.2): the escape switch in
     `read_c_string_or_comment` and the type switch in `write_globals`.  A byte literal is
     not a Rust pattern, so the four `case '\n'/'\r'/'n'/'t'` arms spell the characters as
     their values with C's `case` text in the comment.
  6. C 852-1231's `eof:` label and the forward `goto eof` that leaves the loop nest for the
     cleanup become the labelled `'eof` loop with `break 'eof` (never an early `return`,
     which would skip `ferror`/`fclose`); C's `while (!feof (infile))` test is the loop's
     first statement and its `continue`s are `continue 'eof`.
  7. `static char const commonsym[][8]` (C 653-654, function-local) is the port's
     function-local `static commonsym: [[u8; 8]; 5]` with C's rows as byte strings (the
     `src/coding.rs` `suffixes` precedent), which also keeps C's `8` and the entry count in
     the body.  `static char name[sizeof input_buffer]` (C 869) takes the measured 128,
     because an array length cannot come from another item's `size_of`.
  8. `minargs--`/`maxargs--`/`symnum++`-style updates are the explicit `-= 1`/`+= 1` forms;
     `num_symbols += globals[i].type == SYMBOL;` keeps its shape as the cast of the
     comparison; `while (ident_length-- > 0)` keeps the post-decrement (the failing test
     still decrements, and the trailing `ident_length = 0;` restores it).
- **Shared rows this unit needs (not written here: `docs/ports/*.tsv` is the integrator's).**
  With the shared `macro-body-notes.tsv` as it stands, `--check` reports five
  "does not mention" problems, all of the P-04/2.2 "the token cannot appear in Rust" class:
  `error` and `fatal` need `va_end` added to their existing rows (`va_list`/`va_start` are
  already allowed); `write_c_args` needs `uninit`; `write_globals` needs
  `break,case,default,switch`; `scan_c_stream` needs `goto`.  With those five rows appended
  to a copy of the file (`/tmp/mdf-notes.tsv`) plus `--loose-lines`, `--check` prints `OK`,
  so no other token is missing.  The unit also needs a `line-drift-tolerance.tsv` row
  (`make-docfile  0.40  C 1232 vs the port's 1686 (36.9%): ... total`) — C's dense style
  (one-line `if (…) continue;`, `case …: …; break;`, `do c = getc (infile); while (…)`) costs
  a braced block each in the port (504 brace-only lines against C's 201), and the labelled
  `'eof` loop, the `match` arms, the file-local structs' layout assertions and the
  per-statement comments add lines C does not have; the `lib-src/hexl.rs` (0.55) and
  `lib-src/make-fingerprint.rs` (0.45) rows are the same situation.  36.9% < 0.40, so that
  row makes the drift gate pass.


- **Integration (the integrator's pass).** The five shared `macro-body-notes.tsv` rows the
  ledger asked for (`error`/`fatal` + `va_end`, `write_c_args` + `uninit`, `write_globals` +
  `break,case,default,switch`, `scan_c_stream` + `goto`) and the `line-drift-tolerance.tsv`
  row (0.38 against the measured 36.9 per cent) are in the shared `docs/ports/*.tsv` files now,
  so the check runs against them rather than against a copy: `python3 tools/extract-abi.py
  --check emacs/lib-src/make-docfile.c femacs/lib-src/make-docfile.rs` -> `OK`, and the
  ledger's 30 boxes are checked because of that reading.

## Evidence:

- `python3 tools/extract-abi.py emacs/lib-src/make-docfile.c > docs/ports/make-docfile.md`
  (the ledger itself; 21 functions / 2 structs / 6 file-scope objects / 0 macros / 0 subrs).
- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mdf cargo run -q -p xtask -- build-tu lib-src/make-docfile.c`
  -> `built lib-src/make-docfile.rs -> ../build/femacs/make-docfile.o`
- `python3 tools/extract-abi.py --check emacs/lib-src/make-docfile.c femacs/lib-src/make-docfile.rs`
  -> last line `FAIL …`, 6 problems, all of the shared-row class above:
  `function error does not mention va_end`,
  `function fatal does not mention va_end`,
  `function write_c_args does not mention uninit`,
  `function write_globals does not mention break, case, default, switch`,
  `function scan_c_stream does not mention goto`,
  `line count drifted 36.9%: C 1232 vs Rust 1686`.
- `python3 tools/extract-abi.py --check --macro-notes /tmp/mdf-notes.tsv <same files>`
  -> last line `FAIL …`, one problem: the line-count drift only.
- `python3 tools/extract-abi.py --check --loose-lines --macro-notes /tmp/mdf-notes.tsv <same files>`
  -> `OK emacs/lib-src/make-docfile.c -> femacs/lib-src/make-docfile.rs`.
- `cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name mdf_check
  -C panic=abort -C codegen-units=1 -C opt-level=0
  ../build/femacs/units/make-docfile/_root.rs -o /tmp/mdf.rmeta` -> exit 0 (3 warnings, none
  from this unit: they come from the `unistd.in.rs` mirror's pre-existing declarations).
- `python3 tools/check-cond-eval.py lib-src/make-docfile.c` ->
  `2 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/check-globals.py` ->
  `check-globals: 274 units, 0 DEFVAR_* variables without a definition in their own unit`.
- `python3 tools/check-declarations.py` ->
  `8 declaration sites (0 forbidden, 8 allowed literals), 0 failures`.
- `python3 tools/check-mirror-dupes.py` -> `9 duplicate declaration(s), 1 ratchet row(s),
  8 failure(s)`; every failure is another lane's in-flight pair
  (`femacs/src/xdisp.rs`, `femacs/src/dispnew.rs`, `femacs/src/menu.rs` against
  `femacs/src/include/window.rs`), none names this unit.
- The ledger's boxes were left unchecked by the lane: `--check` was not yet `OK` against
  the shared files, and `extract-abi.py --all` would have counted the unit as in progress
  rather than failing it.  With the shared rows in place the check prints `OK` (see the
  Integration note above), so the 29 boxes are checked against that reading.
